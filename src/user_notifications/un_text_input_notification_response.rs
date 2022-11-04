use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSString,
    objective_c_runtime::{macros::object, traits::FromId},
};

use super::IUNNotificationResponse;

object! {
    /// The user’s response to an actionable notification, including any custom text that the user typed or dictated.
    unsafe pub struct UNTextInputNotificationResponse;
}

impl IUNNotificationResponse for UNTextInputNotificationResponse {}

#[interface_impl(UNNotificationResponse)]
impl UNTextInputNotificationResponse {
    /// The text response provided by the user.
    #[property]
    pub fn user_text(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), userText]) }
    }
}
