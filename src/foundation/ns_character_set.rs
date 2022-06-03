use std::{fmt, ops::Range};

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    id,
    objective_c_runtime::traits::{FromId, PNSObject, ToId},
    utils::to_bool,
};

use super::{ns_coder::NSCoder, traits::INSCharacterSet, unichar, NSData, NSString, UInt, UInt8};

/// A character set containing the characters in Unicode General Categories L*, M*, and N*.
pub struct NSCharacterSet {
    /// The underlying Objective-C object.
    pub ptr: Id<Object>,
}

impl PNSObject for NSCharacterSet {
    fn im_class<'a>() -> &'a Class {
        class!(NSCharacterSet)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isEqual: object] })
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.ptr, hash] }
    }

    fn im_isKindOfClass(&self, aClass: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isKindOfClass: aClass] })
    }

    fn im_isMemberOfClass(&self, aClass: Class) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isMemberOfClass: aClass] })
    }

    fn im_respondsToSelector(&self, aSelector: Sel) -> bool {
        to_bool(unsafe { msg_send![self.ptr, respondsToSelector: aSelector] })
    }

    fn im_conformsToProtocol(&self, aProtocol: Protocol) -> bool {
        to_bool(unsafe { msg_send![self.ptr, conformsToProtocol: aProtocol] })
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, description]) }
    }

    fn ip_debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.ptr, debugDescription]) }
    }

    fn im_performSelector(&self, aSelector: Sel) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: Sel, withObject: id) -> id {
        unsafe { msg_send![self.ptr, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        to_bool(unsafe { msg_send![self.ptr, isProxy] })
    }
}

impl INSCharacterSet for NSCharacterSet {
    fn tp_alphanumericCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), alphanumericCharacterSet])
        }
    }

    fn tp_capitalizedLetterCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                capitalizedLetterCharacterSet
            ])
        }
    }

    fn tp_controlCharacterSet() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), controlCharacterSet]) }
    }

    fn tp_decimalDigitCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), decimalDigitCharacterSet])
        }
    }

    fn tp_decomposableCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), decomposableCharacterSet])
        }
    }

    fn tp_illegalCharacterSet() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), illegalCharacterSet]) }
    }

    fn tp_letterCharacterSet() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), letterCharacterSet]) }
    }

    fn tp_lowercaseLetterCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                lowercaseLetterCharacterSet
            ])
        }
    }

    fn tp_newlineCharacterSet() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), newlineCharacterSet]) }
    }

    fn tp_nonBaseCharacterSet() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), nonBaseCharacterSet]) }
    }

    fn tp_punctuationCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), punctuationCharacterSet])
        }
    }

    fn tp_symbolCharacterSet() -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), symbolCharacterSet]) }
    }

    fn tp_uppercaseLetterCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                uppercaseLetterCharacterSet
            ])
        }
    }

    fn tp_whitespaceAndNewlineCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                whitespaceAndNewlineCharacterSet
            ])
        }
    }

    fn tp_whitespaceCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![class!(NSCharacterSet), whitespaceCharacterSet])
        }
    }

    fn tp_URLFragmentAllowedCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLFragmentAllowedCharacterSet
            ])
        }
    }

    fn tp_URLHostAllowedCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLHostAllowedCharacterSet
            ])
        }
    }

    fn tp_URLPasswordAllowedCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLPasswordAllowedCharacterSet
            ])
        }
    }

    fn tp_URLPathAllowedCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLPathAllowedCharacterSet
            ])
        }
    }

    fn tp_URLQueryAllowedCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLQueryAllowedCharacterSet
            ])
        }
    }

    fn tp_URLUserAllowedCharacterSet() -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                URLUserAllowedCharacterSet
            ])
        }
    }

    fn im_initWithCoder(self, coder: NSCoder) -> Self {
        unsafe {
            let ptr = msg_send![self.ptr, initWithCoder: coder];
            Self::from_id(ptr)
        }
    }

    fn tm_characterSetWithCharactersInString(string: NSString) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                characterSetWithCharactersInString: string
            ])
        }
    }

    fn tm_characterSetWithRange(range: Range<UInt>) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                characterSetWithRange: range
            ])
        }
    }

    fn tm_characterSetWithBitmapRepresentation(data: NSData) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                characterSetWithBitmapRepresentation: data
            ])
        }
    }

    fn tm_characterSetWithContentsOfFile(path: NSString) -> NSCharacterSet {
        unsafe {
            NSCharacterSet::from_id(msg_send![
                class!(NSCharacterSet),
                characterSetWithContentsOfFile: path
            ])
        }
    }

    fn ip_bitmapRepresentation(&self) -> NSData {
        unsafe { NSData::from_id(msg_send![self.ptr, bitmapRepresentation]) }
    }

    fn ip_invertedSet(&self) -> NSCharacterSet {
        unsafe { NSCharacterSet::from_id(msg_send![self.ptr, invertedSet]) }
    }

    fn im_characterIsMember(&self, character: unichar) -> bool {
        unsafe { msg_send![self.ptr, characterIsMember: character] }
    }

    fn im_hasMemberInPlane(&self, plane: UInt8) -> bool {
        unsafe { msg_send![self.ptr, hasMemberInPlane: plane] }
    }

    fn im_isSupersetOfSet(&self, theOther: NSCharacterSet) -> bool {
        unsafe { msg_send![self.ptr, isSupersetOfSet: theOther] }
    }

    fn im_longCharacterIsMember(&self, theLongChar: u32) -> bool {
        unsafe { msg_send![self.ptr, longCharacterIsMember: theLongChar] }
    }
}

impl ToId for NSCharacterSet {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSCharacterSet {
    unsafe fn from_id(id: id) -> Self {
        Self {
            ptr: Id::from_ptr(id),
        }
    }
}

impl fmt::Debug for NSCharacterSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_debugDescription())
    }
}

impl fmt::Display for NSCharacterSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
