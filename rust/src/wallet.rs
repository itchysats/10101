use crate::config;
use crate::config::maker_endpoint;
use crate::config::maker_peer_info;
use crate::config::TCP_TIMEOUT;
use crate::db;
use crate::db::load_payments;
use crate::db::update_ignore_txid;
use crate::lightning;
use crate::lightning::ChannelManager;
use crate::lightning::Flow;
use crate::lightning::HTLCStatus;
use crate::lightning::LightningSystem;
use crate::lightning::NodeInfo;
use crate::lightning::PeerInfo;
use crate::seed::Bip39Seed;
use ::lightning::chain::chaininterface::ConfirmationTarget;
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use bdk::bitcoin;
use bdk::bitcoin::secp256k1::PublicKey;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::Address;
use bdk::bitcoin::Amount;
use bdk::bitcoin::Network;
use bdk::bitcoin::Script;
use bdk::bitcoin::Txid;
use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bdk::wallet::wallet_name_from_descriptor;
use bdk::wallet::AddressIndex;
use bdk::FeeRate;
use bdk::KeychainKind;
use bdk::SignOptions;
use bdk_ldk::ScriptStatus;
use lightning_background_processor::BackgroundProcessor;
use lightning_invoice::Invoice;
use rust_decimal::prelude::FromPrimitive;
use serde::Deserialize;
use serde::Serialize;
use state::Storage;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::time::Duration;
use tokio::task::JoinHandle;

/// Wallet has to be managed by Rust as generics are not support by frb
static WALLET: Storage<Mutex<Wallet>> = Storage::new();

#[derive(Clone)]
pub struct Wallet {
    seed: Bip39Seed,
    pub lightning: LightningSystem,
    network: Network,
}

#[derive(Debug, Clone, Serialize)]
pub struct Balance {
    pub on_chain: OnChain,
    pub off_chain: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OnChain {
    /// Unconfirmed UTXOs generated by a wallet tx
    pub trusted_pending: u64,
    /// Unconfirmed UTXOs received from an external wallet
    pub untrusted_pending: u64,
    /// Confirmed and immediately spendable balance
    pub confirmed: u64,
}

impl Wallet {
    pub fn new(data_dir: &Path) -> Result<Wallet> {
        let network = config::network();
        let electrum_str = config::electrum_url();
        tracing::info!(?network, electrum_str, "Creating the wallet");

        let data_dir = data_dir.join(&network.to_string());
        if !data_dir.exists() {
            std::fs::create_dir(&data_dir)
                .context(format!("Could not create data dir for {network}"))?;
        }
        let seed_path = data_dir.join("seed");
        let seed = Bip39Seed::initialize(&seed_path)?;
        let ext_priv_key = seed.derive_extended_priv_key(network)?;

        let client = Client::new(&electrum_str)?;
        let blockchain = ElectrumBlockchain::from(client);

        let wallet_name = wallet_name_from_descriptor(
            bdk::template::Bip84(ext_priv_key, KeychainKind::External),
            Some(bdk::template::Bip84(ext_priv_key, KeychainKind::Internal)),
            ext_priv_key.network,
            &Secp256k1::new(),
        )?;

        // Create a database (using default sled type) to store wallet data
        let db = bdk::sled::open(data_dir.join("wallet"))?;
        let db = db.open_tree(wallet_name)?;

        let bdk_wallet = bdk::Wallet::new(
            bdk::template::Bip84(ext_priv_key, KeychainKind::External),
            Some(bdk::template::Bip84(ext_priv_key, KeychainKind::Internal)),
            ext_priv_key.network,
            db,
        )?;

        let lightning_wallet = bdk_ldk::LightningWallet::new(Box::new(blockchain), bdk_wallet);

        // Lightning seed needs to be shorter
        let lightning_seed = &seed.seed()[0..32].try_into()?;

        let lightning = lightning::setup(lightning_wallet, network, &data_dir, lightning_seed)?;

        Ok(Wallet {
            lightning,
            seed,
            network,
        })
    }

    pub fn sync(&self) -> Result<()> {
        self.lightning
            .wallet
            .sync(self.lightning.confirmables())
            .map_err(|_| anyhow!("Could lot sync bdk-ldk wallet"))?;
        Ok(())
    }

    pub fn network(&self) -> Network {
        self.network
    }

    pub fn get_balance(&self) -> Result<Balance> {
        let bdk_balance = self.get_bdk_balance()?;
        let ldk_balance = self.get_ldk_balance();
        Ok(Balance {
            // subtract the ldk balance from the bdk balance as this balance is locked in the
            // off chain wallet.
            on_chain: OnChain {
                trusted_pending: bdk_balance.trusted_pending,
                untrusted_pending: bdk_balance.untrusted_pending,
                confirmed: bdk_balance.confirmed,
            },
            off_chain: ldk_balance,
        })
    }

    fn get_bdk_balance(&self) -> Result<bdk::Balance> {
        let balance = self
            .lightning
            .wallet
            .get_balance()
            .map_err(|_| anyhow!("Could not retrieve bdk wallet balance"))?;
        tracing::debug!(%balance, "Wallet balance");
        Ok(balance)
    }

    /// LDK balance is the total sum of money in all open channels
    fn get_ldk_balance(&self) -> u64 {
        self.lightning
            .channel_manager
            .list_channels()
            .iter()
            .map(|details| details.balance_msat / 1000)
            .sum()
    }

    fn get_channel_manager(&self) -> Arc<ChannelManager> {
        self.lightning.channel_manager.clone()
    }

    pub fn get_address(&self) -> Result<bitcoin::Address> {
        let address = self
            .lightning
            .wallet
            .get_wallet()?
            .get_address(AddressIndex::LastUnused)?;
        tracing::debug!(%address, "Current wallet address");
        Ok(address.address)
    }

    /// Run the lightning node
    pub async fn run_ldk(&self) -> Result<BackgroundProcessor> {
        lightning::run_ldk(&self.lightning).await
    }

    /// Run the lightning node
    pub async fn run_ldk_server(
        &self,
        address: SocketAddr,
    ) -> Result<(JoinHandle<()>, BackgroundProcessor)> {
        lightning::run_ldk_server(&self.lightning, address).await
    }

    pub async fn get_bitcoin_tx_history(&self) -> Result<Vec<bdk::TransactionDetails>> {
        let tx_history = self
            .lightning
            .wallet
            .get_wallet()?
            .list_transactions(false)?;

        let ignore_maker_funding = db::load_ignore_txids().await?;
        let ignore_txids = ignore_maker_funding
            .clone()
            .into_iter()
            .map(|(txid, _, _)| txid)
            .collect::<Vec<Txid>>();

        // Ignore the maker's funding tx
        let mut tx_history = tx_history
            .into_iter()
            .filter(|tx_detail| !ignore_txids.contains(&tx_detail.txid))
            .collect::<Vec<_>>();

        for (maker_funding_txid, maker_funding_amount, open_channel_txid) in ignore_maker_funding {
            let open_channel_txid = match open_channel_txid {
                Some(open_channel_txid) => open_channel_txid,
                None => {
                    // Try to extract it from channel
                    let channel_manager = {
                        let lightning = &get_wallet()?.lightning;
                        lightning.channel_manager.clone()
                    };

                    // Try to extract the funding_txid from the first channel
                    let channels = channel_manager.list_channels();
                    let current_channel_details = match channels.first() {
                        Some(channel_details) => channel_details,
                        None => {
                            tracing::warn!("No channel available, but no open-channel txid set for maker funding txid {maker_funding_txid}. The history will likely reflect the maker's funding amount.");
                            continue;
                        }
                    };

                    match current_channel_details.funding_txo {
                        Some(output) => {
                            // Save the funding_txid in the db so we have it persisted for the next
                            // time
                            if let Err(e) =
                                update_ignore_txid(maker_funding_txid, output.txid).await
                            {
                                tracing::warn!("Failed to update the open_channel_txid in the database. This might cause weird issues when displaying the payment history after channel close: {e:#}");
                            }

                            output.txid
                        }
                        None => {
                            // This can happen if the transaction was not picked up by the lightning
                            // wallet yet, but we already saved the expected information to ignored
                            // into the database
                            tracing::debug!("Failed to retrieve channel funding tx, the funding_txo is not available for the channel yet.");
                            continue;
                        }
                    }
                }
            };

            tx_history.iter_mut().for_each(|tx_detail| {
                if tx_detail.txid == open_channel_txid {
                    tx_detail.sent -= maker_funding_amount as u64;
                }
            });
        }

        tracing::trace!(?tx_history, "Transaction history");
        Ok(tx_history)
    }

    pub fn get_node_id(&self) -> PublicKey {
        self.lightning.channel_manager.get_our_node_id()
    }

    pub fn get_script_status(&self, script: Script, txid: Txid) -> Result<ScriptStatus> {
        let script_status = self
            .lightning
            .wallet
            .get_tx_status_for_script(script, txid)
            .map_err(|_| anyhow!("Could not get tx status for script"))?;
        Ok(script_status)
    }

    pub fn send_to_address(&self, send_to: Address, amount: u64) -> Result<Txid> {
        tracing::debug!(address = %send_to, sats = %amount, "Sending to address");

        let wallet = self.lightning.wallet.get_wallet()?;

        let estimated_fee_rate = self
            .lightning
            .wallet
            .estimate_fee(ConfirmationTarget::Normal)
            .map_err(|_| anyhow!("Failed to estimate fee"))?;

        let (mut psbt, _) = {
            let mut builder = wallet.build_tx();

            builder
                .fee_rate(FeeRate::from_sat_per_vb(
                    f32::from_u32(estimated_fee_rate).unwrap_or(1.0),
                ))
                .enable_rbf();

            let script_pubkey = send_to.script_pubkey();
            let balance = wallet.get_balance()?;
            if amount == balance.confirmed {
                builder.drain_wallet().drain_to(script_pubkey);
            } else {
                builder.add_recipient(script_pubkey, amount);
            }

            builder.finish()?
        };

        if !wallet.sign(&mut psbt, SignOptions::default())? {
            bail!("Failed to sign psbt");
        }

        let tx = psbt.extract_tx();

        self.lightning.wallet.broadcast(&tx)?;

        Ok(tx.txid())
    }

    /// Fee recommendation in sats per vbyte.
    pub fn get_fee_recommendation(&self) -> Result<u32> {
        let fee_rate = self
            .lightning
            .wallet
            .estimate_fee(ConfirmationTarget::Normal)
            .map_err(|_| anyhow!("Failed to estimate fee"))?;

        Ok(fee_rate)
    }
}

// XXX: Try not to make this function public - exposing MutexGuard is risky.
// Instead, expose a free function that wraps this in a way that returns what
// you're after.
fn get_wallet() -> Result<MutexGuard<'static, Wallet>> {
    WALLET
        .try_get()
        .context("Wallet uninitialised")?
        .lock()
        .map_err(|_| anyhow!("cannot acquire wallet lock"))
}

/// Boilerplate wrappers for using Wallet with static functions in the library

pub fn init_wallet(data_dir: &Path) -> Result<()> {
    tracing::debug!(?data_dir, "Wallet will be stored on disk");
    WALLET.set(Mutex::new(Wallet::new(data_dir)?));
    Ok(())
}

pub async fn run_ldk() -> Result<BackgroundProcessor> {
    let wallet = { (*get_wallet()?).clone() };
    wallet.run_ldk().await
}

pub async fn run_ldk_server(address: SocketAddr) -> Result<(JoinHandle<()>, BackgroundProcessor)> {
    let wallet = { (*get_wallet()?).clone() };
    wallet.run_ldk_server(address).await
}

pub fn node_id() -> Result<PublicKey> {
    let node_id = get_wallet()?.get_node_id();
    Ok(node_id)
}

pub fn get_balance() -> Result<Balance> {
    get_wallet()?.get_balance()
}

pub fn sync() -> Result<()> {
    tracing::trace!("Wallet sync called");
    get_wallet()?.sync()
}

pub fn network() -> Result<bitcoin::Network> {
    Ok(get_wallet()?.network())
}

pub fn get_address() -> Result<bitcoin::Address> {
    get_wallet()?.get_address()
}

pub async fn get_bitcoin_tx_history() -> Result<Vec<bdk::TransactionDetails>> {
    let wallet = { (*get_wallet()?).clone() };
    let tx_history = wallet.get_bitcoin_tx_history().await?;
    Ok(tx_history)
}

pub fn get_channel_manager() -> Result<Arc<ChannelManager>> {
    Ok(get_wallet()?.get_channel_manager())
}

pub async fn get_lightning_history() -> Result<Vec<LightningTransaction>> {
    let payments = load_payments()
        .await?
        .iter()
        .map(|payment_info| LightningTransaction {
            tx_type: LightningTransactionType::Payment,
            flow: payment_info.flow.clone(),
            sats: Amount::from(payment_info.amt_msat.clone()).to_sat(),
            status: payment_info.status.clone(),
            timestamp: payment_info.updated_timestamp,
        })
        .collect();
    Ok(payments)
}

pub fn get_seed_phrase() -> Result<Vec<String>> {
    let seed_phrase = get_wallet()?.seed.get_seed_phrase();
    Ok(seed_phrase)
}

pub async fn send_lightning_payment(invoice: &str) -> Result<()> {
    let invoice = Invoice::from_str(invoice).context("Could not parse Invoice string")?;
    lightning::send_payment(&invoice).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenChannelRequest {
    /// The taker address where the maker should send the funds to
    pub address_to_fund: Address,

    /// The amount that the taker expects for funding
    ///
    /// This represents the amount of the maker.
    pub fund_amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenChannelResponse {
    pub funding_txid: Txid,
}

pub async fn open_channel(peer_info: PeerInfo, taker_amount: u64) -> Result<()> {
    let maker_amount = taker_amount * 2;
    let channel_capacity = taker_amount + maker_amount;

    let address_to_fund = get_address()?;

    let client = reqwest::Client::builder().timeout(TCP_TIMEOUT).build()?;

    let body = OpenChannelRequest {
        address_to_fund: address_to_fund.clone(),
        fund_amount: maker_amount,
    };

    let endpoint = maker_endpoint();
    let maker_api = format!("{endpoint}/api/channel/open");

    tracing::info!("Sending request to open channel to maker at: {maker_api}");

    let response = client.post(maker_api).json(&body).send().await?;

    if let Err(e) = response.error_for_status_ref() {
        let text = response.text().await?;
        bail!("maker was unable to open a channel due to {e}, {text}")
    }

    let response: OpenChannelResponse = response.json().await?;

    let maker_funding_txid = response.funding_txid;

    if let Err(e) = db::insert_ignore_txid(maker_funding_txid, maker_amount as i64).await {
        tracing::warn!("Failed to insert maker funding tx to be ignored in transaction history, the taker will see the funding tx: {e:#}");
    }

    // We cannot wait indefinitely so we specify how long we wait for the maker funds to arrive in
    // mempool
    let secs_until_we_consider_maker_funding_failed = 600;

    let mut processing_sec_counter = 0;
    let sleep_secs = 5;
    while get_wallet()?.get_script_status(address_to_fund.script_pubkey(), maker_funding_txid)?
        == ScriptStatus::Unseen
    {
        processing_sec_counter += sleep_secs;
        if processing_sec_counter >= secs_until_we_consider_maker_funding_failed {
            bail!("The maker screwed up, the funds did not arrive within {secs_until_we_consider_maker_funding_failed} secs so we cannot open channel");
        }

        tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
    }

    // Open Channel
    let channel_manager = {
        let lightning = &get_wallet()?.lightning;
        lightning.channel_manager.clone()
    };

    lightning::open_channel(
        channel_manager,
        peer_info,
        channel_capacity,
        Some(maker_amount),
    )
    .await
}

pub async fn connect() -> Result<()> {
    let peer_manager = {
        let lightning = &get_wallet()?.lightning;

        lightning.peer_manager.clone()
    };
    let peer_info = maker_peer_info();
    tracing::debug!("Connection with {peer_info}");
    lightning::connect_peer_if_necessary(&peer_info, peer_manager).await?;

    Ok(())
}

pub async fn close_channel(remote_node_id: PublicKey, force: bool) -> Result<()> {
    let channel_manager = {
        let lightning = &get_wallet()?.lightning;

        lightning.channel_manager.clone()
    };

    lightning::close_channel(channel_manager, remote_node_id, force).await?;

    Ok(())
}

pub fn send_to_address(address: Address, amount: u64) -> Result<Txid> {
    get_wallet()?.send_to_address(address, amount)
}

pub fn get_node_info() -> Result<NodeInfo> {
    Ok(get_wallet()?.lightning.node_info())
}

pub async fn create_invoice(
    amount_sats: u64,
    expiry_secs: u32,
    description: String,
) -> Result<String> {
    let (channel_manager, keys_manager, network, logger) = {
        let wallet = get_wallet()?;
        (
            wallet.lightning.channel_manager.clone(),
            wallet.lightning.keys_manager.clone(),
            wallet.lightning.network,
            wallet.lightning.logger.clone(),
        )
    };

    let amount_msat = amount_sats * 1000;
    lightning::create_invoice(
        amount_msat,
        channel_manager,
        keys_manager,
        network,
        description,
        expiry_secs,
        logger,
    )
    .await
}

pub fn get_fee_recommendation() -> Result<u32> {
    get_wallet()?.get_fee_recommendation()
}

pub enum LightningTransactionType {
    Payment,
    Cfd,
}

pub struct LightningTransaction {
    pub tx_type: LightningTransactionType,
    pub flow: Flow,
    pub sats: u64,
    pub status: HTLCStatus,
    pub timestamp: u64,
}
