// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.49.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names

import "bridge_definitions.dart";
import 'dart:convert';
import 'dart:async';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.dart';
export 'bridge_generated.dart';
import 'package:meta/meta.dart';
import 'dart:ffi' as ffi;

class FlutterRustBridgeExamplePlatform
    extends FlutterRustBridgeBase<FlutterRustBridgeExampleWire> {
  FlutterRustBridgeExamplePlatform(ffi.DynamicLibrary dylib)
      : super(FlutterRustBridgeExampleWire(dylib));
// Section: api2wire

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_String(String raw) {
    return api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  @protected
  ffi.Pointer<wire_Point> api2wire_box_autoadd_point(Point raw) {
    final ptr = inner.new_box_autoadd_point_0();
    _api_fill_to_wire_point(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_Size> api2wire_box_autoadd_size(Size raw) {
    final ptr = inner.new_box_autoadd_size_0();
    _api_fill_to_wire_size(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_TreeNode> api2wire_box_autoadd_tree_node(TreeNode raw) {
    final ptr = inner.new_box_autoadd_tree_node_0();
    _api_fill_to_wire_tree_node(raw, ptr.ref);
    return ptr;
  }

  @protected
  ffi.Pointer<wire_list_size> api2wire_list_size(List<Size> raw) {
    final ans = inner.new_list_size_0(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_size(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_list_tree_node> api2wire_list_tree_node(List<TreeNode> raw) {
    final ans = inner.new_list_tree_node_0(raw.length);
    for (var i = 0; i < raw.length; ++i) {
      _api_fill_to_wire_tree_node(raw[i], ans.ref.ptr[i]);
    }
    return ans;
  }

  @protected
  ffi.Pointer<wire_uint_8_list> api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list_0(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }
// Section: api_fill_to_wire

  void _api_fill_to_wire_box_autoadd_point(
      Point apiObj, ffi.Pointer<wire_Point> wireObj) {
    _api_fill_to_wire_point(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_size(
      Size apiObj, ffi.Pointer<wire_Size> wireObj) {
    _api_fill_to_wire_size(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_box_autoadd_tree_node(
      TreeNode apiObj, ffi.Pointer<wire_TreeNode> wireObj) {
    _api_fill_to_wire_tree_node(apiObj, wireObj.ref);
  }

  void _api_fill_to_wire_point(Point apiObj, wire_Point wireObj) {
    wireObj.x = api2wire_f64(apiObj.x);
    wireObj.y = api2wire_f64(apiObj.y);
  }

  void _api_fill_to_wire_size(Size apiObj, wire_Size wireObj) {
    wireObj.width = api2wire_i32(apiObj.width);
    wireObj.height = api2wire_i32(apiObj.height);
  }

  void _api_fill_to_wire_tree_node(TreeNode apiObj, wire_TreeNode wireObj) {
    wireObj.name = api2wire_String(apiObj.name);
    wireObj.children = api2wire_list_tree_node(apiObj.children);
  }
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class FlutterRustBridgeExampleWire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  FlutterRustBridgeExampleWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  FlutterRustBridgeExampleWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();

  void wire_draw_mandelbrot(
    int port_,
    ffi.Pointer<wire_Size> image_size,
    ffi.Pointer<wire_Point> zoom_point,
    double scale,
    int num_threads,
  ) {
    return _wire_draw_mandelbrot(
      port_,
      image_size,
      zoom_point,
      scale,
      num_threads,
    );
  }

  late final _wire_draw_mandelbrotPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64,
              ffi.Pointer<wire_Size>,
              ffi.Pointer<wire_Point>,
              ffi.Double,
              ffi.Int32)>>('wire_draw_mandelbrot');
  late final _wire_draw_mandelbrot = _wire_draw_mandelbrotPtr.asFunction<
      void Function(
          int, ffi.Pointer<wire_Size>, ffi.Pointer<wire_Point>, double, int)>();

  void wire_passing_complex_structs(
    int port_,
    ffi.Pointer<wire_TreeNode> root,
  ) {
    return _wire_passing_complex_structs(
      port_,
      root,
    );
  }

  late final _wire_passing_complex_structsPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_TreeNode>)>>('wire_passing_complex_structs');
  late final _wire_passing_complex_structs = _wire_passing_complex_structsPtr
      .asFunction<void Function(int, ffi.Pointer<wire_TreeNode>)>();

  void wire_returning_structs_with_boxed_fields(
    int port_,
  ) {
    return _wire_returning_structs_with_boxed_fields(
      port_,
    );
  }

  late final _wire_returning_structs_with_boxed_fieldsPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_returning_structs_with_boxed_fields');
  late final _wire_returning_structs_with_boxed_fields =
      _wire_returning_structs_with_boxed_fieldsPtr
          .asFunction<void Function(int)>();

  void wire_off_topic_memory_test_input_array(
    int port_,
    ffi.Pointer<wire_uint_8_list> input,
  ) {
    return _wire_off_topic_memory_test_input_array(
      port_,
      input,
    );
  }

  late final _wire_off_topic_memory_test_input_arrayPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>(
      'wire_off_topic_memory_test_input_array');
  late final _wire_off_topic_memory_test_input_array =
      _wire_off_topic_memory_test_input_arrayPtr
          .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_off_topic_memory_test_output_zero_copy_buffer(
    int port_,
    int len,
  ) {
    return _wire_off_topic_memory_test_output_zero_copy_buffer(
      port_,
      len,
    );
  }

  late final _wire_off_topic_memory_test_output_zero_copy_bufferPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_off_topic_memory_test_output_zero_copy_buffer');
  late final _wire_off_topic_memory_test_output_zero_copy_buffer =
      _wire_off_topic_memory_test_output_zero_copy_bufferPtr
          .asFunction<void Function(int, int)>();

  void wire_off_topic_memory_test_output_vec_u8(
    int port_,
    int len,
  ) {
    return _wire_off_topic_memory_test_output_vec_u8(
      port_,
      len,
    );
  }

  late final _wire_off_topic_memory_test_output_vec_u8Ptr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_off_topic_memory_test_output_vec_u8');
  late final _wire_off_topic_memory_test_output_vec_u8 =
      _wire_off_topic_memory_test_output_vec_u8Ptr
          .asFunction<void Function(int, int)>();

  void wire_off_topic_memory_test_input_vec_of_object(
    int port_,
    ffi.Pointer<wire_list_size> input,
  ) {
    return _wire_off_topic_memory_test_input_vec_of_object(
      port_,
      input,
    );
  }

  late final _wire_off_topic_memory_test_input_vec_of_objectPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_list_size>)>>(
      'wire_off_topic_memory_test_input_vec_of_object');
  late final _wire_off_topic_memory_test_input_vec_of_object =
      _wire_off_topic_memory_test_input_vec_of_objectPtr
          .asFunction<void Function(int, ffi.Pointer<wire_list_size>)>();

  void wire_off_topic_memory_test_output_vec_of_object(
    int port_,
    int len,
  ) {
    return _wire_off_topic_memory_test_output_vec_of_object(
      port_,
      len,
    );
  }

  late final _wire_off_topic_memory_test_output_vec_of_objectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_off_topic_memory_test_output_vec_of_object');
  late final _wire_off_topic_memory_test_output_vec_of_object =
      _wire_off_topic_memory_test_output_vec_of_objectPtr
          .asFunction<void Function(int, int)>();

  void wire_off_topic_memory_test_input_complex_struct(
    int port_,
    ffi.Pointer<wire_TreeNode> input,
  ) {
    return _wire_off_topic_memory_test_input_complex_struct(
      port_,
      input,
    );
  }

  late final _wire_off_topic_memory_test_input_complex_structPtr = _lookup<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Int64, ffi.Pointer<wire_TreeNode>)>>(
      'wire_off_topic_memory_test_input_complex_struct');
  late final _wire_off_topic_memory_test_input_complex_struct =
      _wire_off_topic_memory_test_input_complex_structPtr
          .asFunction<void Function(int, ffi.Pointer<wire_TreeNode>)>();

  void wire_off_topic_memory_test_output_complex_struct(
    int port_,
    int len,
  ) {
    return _wire_off_topic_memory_test_output_complex_struct(
      port_,
      len,
    );
  }

  late final _wire_off_topic_memory_test_output_complex_structPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64, ffi.Int32)>>(
          'wire_off_topic_memory_test_output_complex_struct');
  late final _wire_off_topic_memory_test_output_complex_struct =
      _wire_off_topic_memory_test_output_complex_structPtr
          .asFunction<void Function(int, int)>();

  void wire_off_topic_deliberately_return_error(
    int port_,
  ) {
    return _wire_off_topic_deliberately_return_error(
      port_,
    );
  }

  late final _wire_off_topic_deliberately_return_errorPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_off_topic_deliberately_return_error');
  late final _wire_off_topic_deliberately_return_error =
      _wire_off_topic_deliberately_return_errorPtr
          .asFunction<void Function(int)>();

  void wire_off_topic_deliberately_panic(
    int port_,
  ) {
    return _wire_off_topic_deliberately_panic(
      port_,
    );
  }

  late final _wire_off_topic_deliberately_panicPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_off_topic_deliberately_panic');
  late final _wire_off_topic_deliberately_panic =
      _wire_off_topic_deliberately_panicPtr.asFunction<void Function(int)>();

  ffi.Pointer<wire_Point> new_box_autoadd_point_0() {
    return _new_box_autoadd_point_0();
  }

  late final _new_box_autoadd_point_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_Point> Function()>>(
          'new_box_autoadd_point_0');
  late final _new_box_autoadd_point_0 = _new_box_autoadd_point_0Ptr
      .asFunction<ffi.Pointer<wire_Point> Function()>();

  ffi.Pointer<wire_Size> new_box_autoadd_size_0() {
    return _new_box_autoadd_size_0();
  }

  late final _new_box_autoadd_size_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_Size> Function()>>(
          'new_box_autoadd_size_0');
  late final _new_box_autoadd_size_0 = _new_box_autoadd_size_0Ptr
      .asFunction<ffi.Pointer<wire_Size> Function()>();

  ffi.Pointer<wire_TreeNode> new_box_autoadd_tree_node_0() {
    return _new_box_autoadd_tree_node_0();
  }

  late final _new_box_autoadd_tree_node_0Ptr =
      _lookup<ffi.NativeFunction<ffi.Pointer<wire_TreeNode> Function()>>(
          'new_box_autoadd_tree_node_0');
  late final _new_box_autoadd_tree_node_0 = _new_box_autoadd_tree_node_0Ptr
      .asFunction<ffi.Pointer<wire_TreeNode> Function()>();

  ffi.Pointer<wire_list_size> new_list_size_0(
    int len,
  ) {
    return _new_list_size_0(
      len,
    );
  }

  late final _new_list_size_0Ptr = _lookup<
          ffi.NativeFunction<ffi.Pointer<wire_list_size> Function(ffi.Int32)>>(
      'new_list_size_0');
  late final _new_list_size_0 = _new_list_size_0Ptr
      .asFunction<ffi.Pointer<wire_list_size> Function(int)>();

  ffi.Pointer<wire_list_tree_node> new_list_tree_node_0(
    int len,
  ) {
    return _new_list_tree_node_0(
      len,
    );
  }

  late final _new_list_tree_node_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_list_tree_node> Function(
              ffi.Int32)>>('new_list_tree_node_0');
  late final _new_list_tree_node_0 = _new_list_tree_node_0Ptr
      .asFunction<ffi.Pointer<wire_list_tree_node> Function(int)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list_0(
    int len,
  ) {
    return _new_uint_8_list_0(
      len,
    );
  }

  late final _new_uint_8_list_0Ptr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list_0');
  late final _new_uint_8_list_0 = _new_uint_8_list_0Ptr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>(
          'free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct = _free_WireSyncReturnStructPtr
      .asFunction<void Function(WireSyncReturnStruct)>();
}

class wire_Size extends ffi.Struct {
  @ffi.Int32()
  external int width;

  @ffi.Int32()
  external int height;
}

class wire_Point extends ffi.Struct {
  @ffi.Double()
  external double x;

  @ffi.Double()
  external double y;
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

class wire_list_tree_node extends ffi.Struct {
  external ffi.Pointer<wire_TreeNode> ptr;

  @ffi.Int32()
  external int len;
}

class wire_TreeNode extends ffi.Struct {
  external ffi.Pointer<wire_uint_8_list> name;

  external ffi.Pointer<wire_list_tree_node> children;
}

class wire_list_size extends ffi.Struct {
  external ffi.Pointer<wire_Size> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
