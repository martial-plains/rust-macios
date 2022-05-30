use std::{fmt, ops::Range};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    id,
    objective_c_runtime::traits::PNSObject,
};

use super::{traits::INLTokenizer, NLTokenUnit};

/// A tokenizer that segments natural language text into semantic units.
pub struct NLTokenizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl PNSObject for NLTokenizer {
    fn class<'a>() -> &'a objc::runtime::Class {
        class!(NLTokenizer)
    }

    fn superclass<'a>() -> &'a objc::runtime::Class {
        unsafe { msg_send![Self::class(), superclass] }
    }

    fn isEqual(&self, object: &Self) -> bool {
        unsafe { msg_send![self.obj, isEqual: object] }
    }

    fn hash(&self) -> UInt {
        unsafe { msg_send![self.obj, hash] }
    }

    fn isKindOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isKindOfClass: aClass] }
    }

    fn isMemberOfClass(&self, aClass: objc::runtime::Class) -> bool {
        unsafe { msg_send![self.obj, isMemberOfClass: aClass] }
    }

    fn respondsToSelector(&self, aSelector: objc::runtime::Sel) -> bool {
        unsafe { msg_send![self.obj, respondsToSelector: aSelector] }
    }

    fn conformsToProtocol(&self, aProtocol: objc::runtime::Protocol) -> bool {
        unsafe { msg_send![self.obj, conformsToProtocol: aProtocol] }
    }

    fn description(&self) -> NSString {
        unsafe { msg_send![self.obj, description] }
    }

    fn debugDescription(&self) -> NSString {
        unsafe { msg_send![self.obj, debugDescription] }
    }

    fn performSelector(&self, aSelector: objc::runtime::Sel) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector] }
    }

    fn performSelector_withObject(&self, aSelector: objc::runtime::Sel, withObject: id) -> id {
        unsafe { msg_send![self.obj, performSelector: aSelector withObject: withObject] }
    }

    fn isProxy(&self) -> bool {
        unsafe { msg_send![self.obj, isProxy] }
    }
}

impl INLTokenizer for NLTokenizer {
    /* Creating a Tokenizer
     */

    fn initWithUnit(unit: NLTokenUnit) -> Self {
        unsafe {
            let cls = class!(NLTokenizer);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, initWithUnit: unit];
            Self { obj }
        }
    }

    fn string(&self) -> NSString {
        unsafe { msg_send![&*self.obj, string] }
    }

    fn setString<S>(&self, string: S)
    where
        S: Into<NSString>,
    {
        unsafe { msg_send![self.obj, setString: string.into()] }
    }

    fn setLanguage(&self, language: NSString) {
        unsafe { msg_send![self.obj, setLanguage: language] }
    }

    fn unit(&self) -> NLTokenUnit {
        unsafe { msg_send![self.obj, unit] }
    }

    fn tokenRangeAtIndex(&self, character_index: UInt) -> Range<UInt> {
        unsafe { msg_send![self.obj, tokenRangeAtIndex: character_index] }
    }

    fn tokenRangeForRange(&self, range: Range<UInt>) -> Range<UInt> {
        unsafe { msg_send![self.obj, tokenRangeForRange: range] }
    }
}

impl fmt::Debug for NLTokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.debugDescription())
    }
}

impl fmt::Display for NLTokenizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}
