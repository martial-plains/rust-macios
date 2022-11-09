use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{foundation::NSString, object, objective_c_runtime::traits::FromId};

use super::{IUNNotificationAction, UNNotificationActionIcon, UNNotificationActionOptions};

object! {
    unsafe pub struct UNTextInputNotificationAction;
}

impl IUNNotificationAction for UNTextInputNotificationAction {}

#[interface_impl(UNNotificationAction)]
impl UNTextInputNotificationAction {
    /* Essentials
     */

    /// Creates an action object that accepts text input from the user.
    #[method]
    pub fn action_with_identifier_title_options_text_input_button_title_text_input_placeholder(
        identifier: NSString,
        title: NSString,
        options: &[UNNotificationActionOptions],
        text_input_button_title: NSString,
        text_input_placeholder: NSString,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), actionWithIdentifier: identifier title: title options: options textInputButtonTitle: text_input_button_title textInputPlaceholder: text_input_placeholder],
            )
        }
    }

    /// Creates an action object that accepts text input from the user.
    #[method]
    pub fn action_with_identifier_title_options_icon_text_input_button_title_text_input_placeholder(
        identifier: NSString,
        title: NSString,
        options: &[UNNotificationActionOptions],
        icon: UNNotificationActionIcon,
        text_input_button_title: NSString,
        text_input_placeholder: NSString,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), actionWithIdentifier: identifier title: title options: options icon: icon textInputButtonTitle: text_input_button_title textInputPlaceholder: text_input_placeholder],
            )
        }
    }

    /* Getting Information
     */

    /// The localized title of the text input button that the system displays to the user.
    #[property]
    pub fn text_input_button_title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), textInputButtonTitle]) }
    }

    /// The placeholder text that the system localizes and displays in the text input field.
    #[property]
    pub fn text_input_placeholder(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), textInputPlaceholder]) }
    }
}
