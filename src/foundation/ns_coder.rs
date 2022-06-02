use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
    utils::to_bool,
};

use super::{traits::INSCoder, NSDecodingFailurePolicy};

/// An abstract class that serves as the basis for objects that enable archiving and distribution of other objects.
pub struct NSCoder {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSCoder {
    fn class<'a>() -> &'a Class {
        class!(NSCoder)
    }

    fn superclass<'a>() -> &'a Class {
        unsafe { msg_send![Self::class(), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isEqual: object] })
    }

    fn hash(&self) -> UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn isKindOfClass(&self, aClass: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isKindOfClass: aClass] })
    }

    fn isMemberOfClass(&self, aClass: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isMemberOfClass: aClass] })
    }

    fn respondsToSelector(&self, aSelector: Sel) -> bool {
        to_bool(unsafe { msg_send![self.ptr, respondsToSelector: aSelector] })
    }

    fn conformsToProtocol(&self, aProtocol: Protocol) -> bool {
        to_bool(unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] })
    }

    fn description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn performSelector(&self, aSelector: Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn performSelector_withObject(&self, aSelector: Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn isProxy(&self) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isProxy] })
    }
}

impl INSCoder for NSCoder {
    fn ip_allowsKeyedCoding(&self) -> bool {
        to_bool(unsafe { msg_send![self.ptr, allowsKeyedCoding] })
    }

    fn im_containsValueForKey<K>(&self, key: K) -> bool
    where
        K: Into<NSString>,
    {
        to_bool(unsafe { msg_send![self.ptr, containsValueForKey: key.into()] })
    }

    fn ip_decodingFailurePolicy(&self) -> NSDecodingFailurePolicy {
        unsafe { msg_send![self.ptr, decodingFailurePolicy] }
    }

    fn im_encodeArrayOfObjCType_count_at(
        &self,
        type_: *const libc::c_char,
        count: usize,
        array: *const libc::c_void,
    ) {
        unsafe {
            msg_send![
                self.ptr,
                encodeArrayOfObjCType: type_
                count: count
                at: array
            ]
        }
    }

    fn im_encodeBool_forKey<K>(&self, value: bool, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeBool: value forKey: key.into()] }
    }

    fn im_encodeBycopyObject<T>(&self, object: T)
    where
        T: PNSObject + ToId,
    {
        unsafe { msg_send![self.ptr, encodeBycopyObject: object.to_id()] }
    }

    fn im_encodeByrefObject<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.ptr, encodeByrefObject: object] }
    }

    fn im_encodeBytes_length(&self, bytes: *const libc::c_void, length: UInt) {
        unsafe { msg_send![self.ptr, encodeBytes: bytes length: length] }
    }

    fn im_encodeBytes_length_forKey<K>(&self, bytes: *const super::UInt8, length: UInt, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeBytes: bytes length: length forKey: key.into()] }
    }

    fn im_encodeConditionalObject<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.ptr, encodeConditionalObject: object] }
    }

    fn im_encodeConditionalObject_forKey<T, K>(&self, object: T, key: K)
    where
        T: PNSObject,
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeConditionalObject: object forKey: key.into()] }
    }

    fn im_encodeDataObject<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.ptr, encodeDataObject: object] }
    }

    fn im_encodeDouble_forKey<K>(&self, value: libc::c_double, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeDouble: value forKey: key.into()] }
    }

    fn im_encodeFloat_forKey<K>(&self, value: libc::c_float, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeFloat: value forKey: key.into()] }
    }

    fn im_encodeInt_forKey<K>(&self, value: super::Int, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeInt: value forKey: key.into()] }
    }

    fn im_encodeInteger_forKey<K>(&self, value: super::Int, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeInteger: value forKey: key.into()] }
    }

    fn im_encodeInt32_forKey<K>(&self, value: super::Int32, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeInt32: value forKey: key.into()] }
    }

    fn im_encodeInt64_forKey<K>(&self, value: super::Int64, key: K)
    where
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeInt64: value forKey: key.into()] }
    }

    fn im_encodeObject<T>(&self, object: T)
    where
        T: PNSObject,
    {
        unsafe { msg_send![self.ptr, encodeObject: object] }
    }

    fn im_encodeObject_forKey<T, K>(&self, object: T, key: K)
    where
        T: PNSObject,
        K: Into<NSString>,
    {
        unsafe { msg_send![self.ptr, encodeObject: object forKey: key.into()] }
    }
}

impl fmt::Debug for NSCoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.debugDescription())
    }
}

impl fmt::Display for NSCoder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl ToId for NSCoder {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSCoder {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
