// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.36.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// The type `UserInfoParams` is not used by any `pub` functions, thus it is ignored.

String greet({required String name, dynamic hint}) =>
    RustLib.instance.api.crateApiSimpleGreet(name: name, hint: hint);

String transact({dynamic hint}) =>
    RustLib.instance.api.crateApiSimpleTransact(hint: hint);
