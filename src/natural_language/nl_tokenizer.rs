use std::{fmt, ops::Range};

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    id,
    objective_c_runtime::traits::{FromId, PNSObject},
};

use super::{traits::INLTokenizer, NLTokenUnit};

/// A tokenizer that segments natural language text into semantic units.
pub struct NLTokenizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl PNSObject for NLTokenizer {
    fn im_class<'a>() -> &'a Class {
        class!(NLTokenizer)
    }

    fn im_isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn im_isKindOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: aClass] }
    }

    fn im_isMemberOfClass(&self, aClass: Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: aClass] }
    }

    fn im_respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: aSelector] }
    }

    fn im_conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: aProtocol] }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, description]) }
    }

    fn ip_debugDescription(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.obj, debugDescription]) }
    }

    fn im_performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector] }
    }

    fn im_performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector withObject: withObject] }
    }

    fn im_isProxy(&self) -> bool {
        unsafe { msg_send![self.obj, isProxy] }
    }
}

impl INLTokenizer for NLTokenizer {
    /* Creating a Tokenizer
     */

    fn im_initWithUnit(unit: NLTokenUnit) -> Self {
        unsafe {
            let cls = class!(NLTokenizer);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, initWithUnit: unit];
            Self { obj }
        }
    }

    fn ip_string(&self) -> NSString {
        unsafe { msg_send![&*self.obj, string] }
    }

    fn ip_setString<S>(&self, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.obj, setString: string.into()] }
    }

    fn im_setLanguage(&self, language: NSString) {
        unsafe { msg_send![self.obj, setLanguage: language] }
    }

    fn ip_unit(&self) -> NLTokenUnit {
        unsafe { msg_send![self.obj, unit] }
    }

    fn im_tokenRangeAtIndex(&self, character_index: UInt) -> Range<UInt> {
        unsafe { msg_send![self.obj, tokenRangeAtIndex: character_index] }
    }

    fn im_tokenRangeForRange(&self, range: Range<UInt>) -> Range<UInt> {
        unsafe { msg_send![self.obj, tokenRangeForRange: range] }
    }
}

impl fmt::Debug for NLTokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.ip_debugDescription())
    }
}

impl fmt::Display for NLTokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
