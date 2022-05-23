use std::{fmt, ops::Range};

use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use objc_id::Id;

use crate::{
    foundation::{String, UInt},
    id,
    objective_c_runtime::traits::NSObject,
};

use super::{traits::NLTokenizer as t_NLTokenizer, NLTokenUnit};

/// A tokenizer that segments natural language text into semantic units.
pub struct NLTokenizer {
    /// The underlying Objective-C object.
    pub obj: Id<Object>,
}

impl NSObject for NLTokenizer {
    fn init() -> Self {
        unsafe {
            let cls = class!(NLTokenizer);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, init];
            Self { obj }
        }
    }

    fn to_id(mut self) -> id {
        &mut *self.obj
    }

    fn from_id(obj: id) -> Self {
        Self {
            obj: unsafe { Id::from_ptr(obj) },
        }
    }

    fn description(&self) -> String {
        let obj: id = unsafe { msg_send![&*self.obj, description] };
        String::from_id(obj)
    }

    fn debug_description(&self) -> String {
        let obj: id = unsafe { msg_send![&*self.obj, debugDescription] };
        String::from_id(obj)
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
        write!(f, "<{}>", self.debug_description())
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

    fn init_with_unit(unit: NLTokenUnit) -> Self {
        unsafe {
            let cls = class!(NLTokenizer);
            let obj: *mut Object = msg_send![cls, new];
            let obj = msg_send![obj, initWithUnit: unit];
            Self { obj }
        }
    }

    fn string(&self) -> String {
        unsafe { msg_send![&*self.obj, string] }
    }

    fn set_string<S>(&self, string: S)
    where
        S: Into<String>,
    {
        unsafe { msg_send![self.obj, setString: string.into()] }
    }

    fn set_language(&self, language: String) {
        unsafe { msg_send![self.obj, setLanguage: language] }
    }

    fn unit(&self) -> NLTokenUnit {
        unsafe { msg_send![self.obj, unit] }
    }

    fn token_range_at_index(&self, character_index: UInt) -> Range<UInt> {
        unsafe { msg_send![self.obj, tokenRangeAtIndex: character_index] }
    }

    fn token_range_for_range(&self, range: Range<UInt>) -> Range<UInt> {
        unsafe { msg_send![self.obj, tokenRangeForRange: range] }
    }
}
