use libc::{c_char, c_double, c_float, c_void};

use crate::{
    foundation::{Int, Int32, Int64, NSDecodingFailurePolicy, NSString, UInt, UInt8},
    objective_c_runtime::traits::{PNSObject, ToId},
};

/// An abstract class that serves as the basis for objects that enable archiving and distribution of other objects.
pub trait INSCoder: PNSObject {
    /* Inspecting a Coder
     */

    /// Returns a Boolean value that indicates whether an encoded value is available for a string.
    fn ip_allowsKeyedCoding(&self) -> bool;

    /// Returns a Boolean value that indicates whether an encoded value is available for a string.
    fn im_containsValueForKey<K>(&self, key: K) -> bool
    where
        K: Into<NSString>;

    /// The action the coder should take when decoding fails.
    fn ip_decodingFailurePolicy(&self) -> NSDecodingFailurePolicy;

    /* Encoding General Data
     */

    /// Encodes an array of the given Objective-C type, provided the number of items and a pointer.
    fn im_encodeArrayOfObjCType_count_at(
        &self,
        type_: *const c_char,
        count: usize,
        array: *const c_void,
    );

    /// Encodes a Boolean value and associates it with the string key.
    fn im_encodeBool_forKey<K>(&self, value: bool, key: K)
    where
        K: Into<NSString>;

    /// An encoding method for subclasses to override such that it creates a copy, rather than a proxy, when decoded.
    fn im_encodeBycopyObject<T>(&self, object: T)
    where
        T: PNSObject + ToId;

    /// An encoding method for subclasses to override such that it creates a proxy, rather than a copy, when decoded.
    fn im_encodeByrefObject<T>(&self, object: T)
    where
        T: PNSObject;

    /// Encodes a buffer of data of an unspecified type.
    fn im_encodeBytes_length(&self, bytes: *const c_void, length: UInt);

    /// Encodes a buffer of data, given its length and a pointer, and associates it with a string key.
    fn im_encodeBytes_length_forKey<K>(&self, bytes: *const UInt8, length: UInt, key: K)
    where
        K: Into<NSString>;

    /// An encoding method for subclasses to override to conditionally encode an object, preserving common references to it.
    fn im_encodeConditionalObject<T>(&self, object: T)
    where
        T: PNSObject;

    /// An encoding method for subclasses to override to conditionally encode an object, preserving common references to it, only if it has been unconditionally encoded.
    fn im_encodeConditionalObject_forKey<T, K>(&self, object: T, key: K)
    where
        T: PNSObject,
        K: Into<NSString>;

    /// Encodes a given data object.
    fn im_encodeDataObject<T>(&self, object: T)
    where
        T: PNSObject;

    /// Encodes a double-precision floating point value and associates it with the string key.
    fn im_encodeDouble_forKey<K>(&self, value: c_double, key: K)
    where
        K: Into<NSString>;

    /// Encodes a floating point value and associates it with the string key.
    fn im_encodeFloat_forKey<K>(&self, value: c_float, key: K)
    where
        K: Into<NSString>;

    /// Encodes a C integer value and associates it with the string key.
    fn im_encodeInt_forKey<K>(&self, value: Int, key: K)
    where
        K: Into<NSString>;

    /// Encodes an integer value and associates it with the string key.
    fn im_encodeInteger_forKey<K>(&self, value: Int, key: K)
    where
        K: Into<NSString>;

    /// Encodes a 32-bit integer value and associates it with the string key.
    fn im_encodeInt32_forKey<K>(&self, value: Int32, key: K)
    where
        K: Into<NSString>;

    /// Encodes a 64-bit integer value and associates it with the string key.
    fn im_encodeInt64_forKey<K>(&self, value: Int64, key: K)
    where
        K: Into<NSString>;

    /// Encodes an object.
    fn im_encodeObject<T>(&self, object: T)
    where
        T: PNSObject;

    /// Encodes an object and associates it with the string key.
    fn im_encodeObject_forKey<T, K>(&self, object: T, key: K)
    where
        T: PNSObject,
        K: Into<NSString>;
}
