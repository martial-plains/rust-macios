pub(crate) use crate::objective_c_runtime::macros::object;

use super::traits::{INSButton, INSControl, INSResponder, INSView};

object! {
    /// A control that defines an area on the screen that a user clicks to trigger an action.
    unsafe pub struct NSButton;
}

impl INSResponder for NSButton {}

impl INSView for NSButton {}

impl INSControl for NSButton {}

impl INSButton for NSButton {}
