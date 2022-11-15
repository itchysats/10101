use http_api_problem::HttpApiProblem;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Price(u64);

impl From<u64> for Price {
    fn from(val: u64) -> Self {
        Self(val)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    bid: Price,
    ask: Price,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offers {
    long: Offer,
    short: Offer,
    index_price: Price,
}

#[rocket::get("/offers")]
pub async fn get_offers() -> Result<Json<Offers>, HttpApiProblem> {
    // TODO: Use real values instead of hard-coding

    let long = Offer {
        bid: 19000.into(),
        ask: 21000.into(),
    };

    // Different values only to ensure we're fetching the correct ones
    let short = Offer {
        bid: 20500.into(),
        ask: 19500.into(),
    };

    let index_price = 19750.into();

    let offers = Offers {
        long,
        short,
        index_price,
    };

    Ok(Json(offers))
}
