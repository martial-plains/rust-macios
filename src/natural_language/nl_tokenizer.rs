use std::{fmt, ops::Range};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    id,
    objective_c_runtime::traits::PNSObject,
};

use super::{traits::t_NLTokenizer, NLTokenUnit};

/// A tokenizer that segments natural language text into semantic units.
pub struct NLTokenizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl PNSObject for NLTokenizer {
    fn new() -> Self {
        unsafe {
            let cls = class!(NLTokenizer);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, init];
            Self { obj }
        }
    }

    fn toId(mut self) -> id {
        &mut *self.obj
    }

    unsafe fn fromId(obj: id) -> Self {
        Self {
            obj: Id::from_ptr(obj),
        }
    }

    fn description(&self) -> NSString {
        let obj: id = unsafe { msg_send![&*self.obj, description] };
        unsafe { NSString::fromId(obj) }
    }

    fn debugDescription(&self) -> NSString {
        let obj: id = unsafe { msg_send![&*self.obj, debugDescription] };
        unsafe { NSString::fromId(obj) }
    }

    fn retain(&self) -> Self {
        unsafe {
            let obj: *mut Object = msg_send![&*self.obj, retain];
            Self {
                obj: Id::from_ptr(obj),
            }
        }
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

impl t_NLTokenizer for NLTokenizer {
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
