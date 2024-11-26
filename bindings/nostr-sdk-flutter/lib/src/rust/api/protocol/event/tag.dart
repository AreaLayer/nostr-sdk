// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.6.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `deref`, `from`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<_Tag>>
abstract class Tag implements RustOpaqueInterface {
  /// Get array of strings
  List<String> asVec();

  /// Return the **first** tag value (index `1`), if exists.
  String? content();

  /// Check if it's a protected event tag
  ///
  /// <https://github.com/nostr-protocol/nips/blob/master/70.md>
  bool isProtected();

  /// Check if is a standard event tag with `reply` marker
  bool isReply();

  /// Check if is a standard event tag with `root` marker
  bool isRoot();

  /// Get tag kind
  String kind();

  /// Parse tag
  ///
  /// Return error if the tag is empty!
  static Tag parse({required List<String> tag}) =>
      RustLib.instance.api.crateApiProtocolEventTagTagParse(tag: tag);

  /// Get array of strings
  List<String> toVec();
}