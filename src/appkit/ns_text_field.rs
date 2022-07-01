use objc::{msg_send, sel, sel_impl};

use crate::objective_c_runtime::traits::{FromId, PNSObject};

use super::{
    object,
    traits::{INSControl, INSResponder, INSTextField, INSView},
};

object! {
    /// Text the user can select or edit to send an action message to a target when the user presses the Return key.
    unsafe pub struct NSTextField;
}

impl INSResponder for NSTextField {}

impl INSView for NSTextField {}

impl INSControl for NSTextField {}

impl INSTextField for NSTextField {}

impl Clone for NSTextField {
    fn clone(&self) -> Self {
        unsafe { Self::from_id(msg_send![self.im_self(), retain]) }
    }
}
