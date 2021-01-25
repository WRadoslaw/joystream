// source: proto/Channel.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

var jspb = require('google-protobuf')
var goog = jspb
var global = Function('return this')()

goog.exportSymbol('proto.ChannelMetadata', null, global)
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.ChannelMetadata = function (opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null)
}
goog.inherits(proto.ChannelMetadata, jspb.Message)
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.ChannelMetadata.displayName = 'proto.ChannelMetadata'
}

if (jspb.Message.GENERATE_TO_OBJECT) {
  /**
   * Creates an object representation of this proto.
   * Field names that are reserved in JavaScript and will be renamed to pb_name.
   * Optional fields that are not set will be set to undefined.
   * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
   * For the list of reserved names please see:
   *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
   * @param {boolean=} opt_includeInstance Deprecated. whether to include the
   *     JSPB instance for transitional soy proto support:
   *     http://goto/soy-param-migration
   * @return {!Object}
   */
  proto.ChannelMetadata.prototype.toObject = function (opt_includeInstance) {
    return proto.ChannelMetadata.toObject(opt_includeInstance, this)
  }

  /**
   * Static version of the {@see toObject} method.
   * @param {boolean|undefined} includeInstance Deprecated. Whether to include
   *     the JSPB instance for transitional soy proto support:
   *     http://goto/soy-param-migration
   * @param {!proto.ChannelMetadata} msg The msg instance to transform.
   * @return {!Object}
   * @suppress {unusedLocalVariables} f is only used for nested messages
   */
  proto.ChannelMetadata.toObject = function (includeInstance, msg) {
    var f,
      obj = {
        title: (f = jspb.Message.getField(msg, 1)) == null ? undefined : f,
        description: (f = jspb.Message.getField(msg, 2)) == null ? undefined : f,
        isPublic: (f = jspb.Message.getBooleanField(msg, 3)) == null ? undefined : f,
        language: (f = jspb.Message.getField(msg, 4)) == null ? undefined : f,
      }

    if (includeInstance) {
      obj.$jspbMessageInstance = msg
    }
    return obj
  }
}

/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.ChannelMetadata}
 */
proto.ChannelMetadata.deserializeBinary = function (bytes) {
  var reader = new jspb.BinaryReader(bytes)
  var msg = new proto.ChannelMetadata()
  return proto.ChannelMetadata.deserializeBinaryFromReader(msg, reader)
}

/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.ChannelMetadata} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.ChannelMetadata}
 */
proto.ChannelMetadata.deserializeBinaryFromReader = function (msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break
    }
    var field = reader.getFieldNumber()
    switch (field) {
      case 1:
        var value = /** @type {string} */ (reader.readString())
        msg.setTitle(value)
        break
      case 2:
        var value = /** @type {string} */ (reader.readString())
        msg.setDescription(value)
        break
      case 3:
        var value = /** @type {boolean} */ (reader.readBool())
        msg.setIsPublic(value)
        break
      case 4:
        var value = /** @type {number} */ (reader.readInt32())
        msg.setLanguage(value)
        break
      default:
        reader.skipField()
        break
    }
  }
  return msg
}

/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.ChannelMetadata.prototype.serializeBinary = function () {
  var writer = new jspb.BinaryWriter()
  proto.ChannelMetadata.serializeBinaryToWriter(this, writer)
  return writer.getResultBuffer()
}

/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.ChannelMetadata} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.ChannelMetadata.serializeBinaryToWriter = function (message, writer) {
  var f = undefined
  f = /** @type {string} */ (jspb.Message.getField(message, 1))
  if (f != null) {
    writer.writeString(1, f)
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 2))
  if (f != null) {
    writer.writeString(2, f)
  }
  f = /** @type {boolean} */ (jspb.Message.getField(message, 3))
  if (f != null) {
    writer.writeBool(3, f)
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 4))
  if (f != null) {
    writer.writeInt32(4, f)
  }
}

/**
 * optional string title = 1;
 * @return {string}
 */
proto.ChannelMetadata.prototype.getTitle = function () {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ''))
}

/**
 * @param {string} value
 * @return {!proto.ChannelMetadata} returns this
 */
proto.ChannelMetadata.prototype.setTitle = function (value) {
  return jspb.Message.setField(this, 1, value)
}

/**
 * Clears the field making it undefined.
 * @return {!proto.ChannelMetadata} returns this
 */
proto.ChannelMetadata.prototype.clearTitle = function () {
  return jspb.Message.setField(this, 1, undefined)
}

/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.ChannelMetadata.prototype.hasTitle = function () {
  return jspb.Message.getField(this, 1) != null
}

/**
 * optional string description = 2;
 * @return {string}
 */
proto.ChannelMetadata.prototype.getDescription = function () {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ''))
}

/**
 * @param {string} value
 * @return {!proto.ChannelMetadata} returns this
 */
proto.ChannelMetadata.prototype.setDescription = function (value) {
  return jspb.Message.setField(this, 2, value)
}

/**
 * Clears the field making it undefined.
 * @return {!proto.ChannelMetadata} returns this
 */
proto.ChannelMetadata.prototype.clearDescription = function () {
  return jspb.Message.setField(this, 2, undefined)
}

/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.ChannelMetadata.prototype.hasDescription = function () {
  return jspb.Message.getField(this, 2) != null
}

/**
 * optional bool is_public = 3;
 * @return {boolean}
 */
proto.ChannelMetadata.prototype.getIsPublic = function () {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 3, false))
}

/**
 * @param {boolean} value
 * @return {!proto.ChannelMetadata} returns this
 */
proto.ChannelMetadata.prototype.setIsPublic = function (value) {
  return jspb.Message.setField(this, 3, value)
}

/**
 * Clears the field making it undefined.
 * @return {!proto.ChannelMetadata} returns this
 */
proto.ChannelMetadata.prototype.clearIsPublic = function () {
  return jspb.Message.setField(this, 3, undefined)
}

/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.ChannelMetadata.prototype.hasIsPublic = function () {
  return jspb.Message.getField(this, 3) != null
}

/**
 * optional int32 language = 4;
 * @return {number}
 */
proto.ChannelMetadata.prototype.getLanguage = function () {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0))
}

/**
 * @param {number} value
 * @return {!proto.ChannelMetadata} returns this
 */
proto.ChannelMetadata.prototype.setLanguage = function (value) {
  return jspb.Message.setField(this, 4, value)
}

/**
 * Clears the field making it undefined.
 * @return {!proto.ChannelMetadata} returns this
 */
proto.ChannelMetadata.prototype.clearLanguage = function () {
  return jspb.Message.setField(this, 4, undefined)
}

/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.ChannelMetadata.prototype.hasLanguage = function () {
  return jspb.Message.getField(this, 4) != null
}

goog.object.extend(exports, proto)
