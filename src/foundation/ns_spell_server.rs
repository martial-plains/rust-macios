use std::sync::Once;

use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};

use objc_id::Id;

use crate::{
    objective_c_runtime::{
        self, id,
        macros::interface_impl,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

use super::NSString;

pub static NSSPELLSERVER_PTR: &str = "rstNSSpellServerPtr";

/// A server that your app uses to provide a spell checker service to other apps running in the system.
pub struct NSSpellServer {
    ptr: Id<Object>,
}

impl PNSObject for NSSpellServer {
    fn m_class<'a>() -> &'a Class {
        static mut NSSPELL_SERVER_CLASS: *const Class = 0 as *const Class;
        static INIT: Once = Once::new();

        INIT.call_once(|| unsafe {
            let superclass = class!(NSApplication);
            let decl = ClassDecl::new("RSTNSSpellServer", superclass).unwrap();
            NSSPELL_SERVER_CLASS = decl.register();
        });

        unsafe { &*NSSPELL_SERVER_CLASS }
    }

    fn m_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl core::fmt::Debug for NSSpellServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        std::write!(f, "{}", self.p_debug_description())
    }
}

impl core::fmt::Display for NSSpellServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        std::write!(f, "{}", self.p_description())
    }
}

impl ToId for NSSpellServer {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSSpellServer {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

#[interface_impl(NSObject)]
impl NSSpellServer {
    /*  Configuring Spelling Servers */

    /// Returns the receiver’s delegate.
    #[property]
    pub fn delegate(&self) -> id {
        unsafe { msg_send![self.m_self(), delegate] }
    }

    /// Sets the receiver's delegate.

    /* Providing Spelling Services
     */

    /// Notifies the receiver of a language your spelling checker can check.
    #[method]
    pub fn register_language_by_vendor(&mut self, language: &NSString, vendor: &NSString) {
        unsafe {
            msg_send![self.m_self(), registerLanguage: language.m_self() byVendor: vendor.m_self()]
        }
    }

    /// Causes the receiver to start listening for spell-checking requests.
    #[method]
    pub fn run(&mut self) {
        unsafe { msg_send![self.m_self(), run] }
    }

    /* Managing the Spell-Checking Process
     */

    /// Indicates whether a given word is in the user’s list of learned words or the document’s list of words to ignore.
    #[method]
    pub fn is_word_in_user_dictionaries_case_sensitive(&self, word: &NSString, flag: bool) -> bool {
        unsafe {
            to_bool(
                msg_send![self.m_self(), isWordInUserDictionaries: word.m_self() caseSensitive: flag],
            )
        }
    }
}

unsafe impl objective_c_runtime::Encode for NSSpellServer {
    fn encode() -> objc::Encoding {
        unsafe { objective_c_runtime::Encoding::from_str("@") }
    }
}

extern "C" {
    /* Grammatical-Analysis Details
     */

    /// The value for the [`NSGrammarRange`] dictionary key should be an NSValue containing an NSRange, a subrange of the sentence range used as the return value, whose location should be an offset from the beginning of the sentence--so, for example, an NSGrammarRange for the first four characters of the overall sentence range should be {0, 4}. If the NSGrammarRange key is not present in the dictionary it is assumed to be equal to the overall sentence range.
    pub static NSGrammarRange: NSString;

    /// The value for the [`NSGrammarUserDescription`] dictionary key should be an NSString containing descriptive text about that range, to be presented directly to the user; it is intended that the user description should provide enough information to allow the user to correct the problem. It is recommended that NSGrammarUserDescription be supplied in all cases, however, NSGrammarUserDescription or NSGrammarCorrections must be supplied in order for correction guidance to be presented to the user.
    pub static NSGrammarUserDescription: NSString;

    /// The value for the [`NSGrammarCorrections`] key should be an NSArray of NSStrings representing potential substitutions to correct the problem, but it is expected that this may not be available in all cases. NSGrammarUserDescription or NSGrammarCorrections must be supplied in order for correction guidance to be presented to the user.
    pub static NSGrammarCorrections: NSString;
}
