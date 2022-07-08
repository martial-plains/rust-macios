use crate::{
    foundation::traits::INSLocale,
    objective_c_runtime::{
        id,
        macros::object,
        traits::{FromId, PNSObject},
    },
};

/// The directions that a language may take across a page of text.
#[repr(usize)]
#[derive(Debug)]
pub enum LanguageDirection {
    /// The direction of the language is unknown.
    Unknown = 0,
    /// The language direction is from left to right.
    LeftToRight = 1,
    /// The language direction is from right to left.
    RightToLeft = 2,
    /// The language direction is from top to bottom.
    TopToBottom = 3,
    /// The language direction is from bottom to top.
    BottomToTop = 4,
}

object! {
    /// The `Locale` class provides information about the user’s locale and formatting preferences.
    unsafe pub struct NSLocale;
}

impl INSLocale for NSLocale {}

impl From<NSLocale> for id {
    /// Converts the `Locale` into an `Object`.
    fn from(val: NSLocale) -> Self {
        unsafe { &mut *val.im_self() }
    }
}

impl From<id> for NSLocale {
    /// Converts the `Object` into a `Locale`.
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(val: id) -> Self {
        unsafe { Self::from_id(val) }
    }
}
