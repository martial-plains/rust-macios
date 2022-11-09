use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSString,
    object,
    objective_c_runtime::traits::{FromId, PNSObject},
    utils::to_optional,
};

use super::UNNotificationActionIcon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u64)]
pub enum UNNotificationActionOptions {
    None = 0,
    AuthenticationRequired = (1 << 0),
    Destructive = (1 << 1),
    Foreground = (1 << 2),
}

object! {
    unsafe pub struct UNNotificationAction;
}

#[interface_impl(NSObject)]
impl UNNotificationAction {
    /* Essentials
     */

    /// Creates an action object by using the specified title and options.
    #[method]
    pub fn action_with_identifier_title_options(
        identifier: NSString,
        title: NSString,
        options: &[UNNotificationActionOptions],
    ) -> Self
    where
        Self: Sized + FromId,
    {
        let options = options.iter().fold(0, |init, option| init | *option as u64);

        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), actionWithIdentifier: identifier title: title options: options],
            )
        }
    }

    #[method]
    pub fn action_with_identifier_title_options_icon(
        identifier: NSString,
        title: NSString,
        options: &[UNNotificationActionOptions],
        icon: UNNotificationActionIcon,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        let options = options.iter().fold(0, |init, option| init | *option as u64);

        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), actionWithIdentifier: identifier title: title options: options icon: icon],
            )
        }
    }

    /* Getting Information
     */

    /// The unique string that your app uses to identify the action.    
    #[property]
    pub fn identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }

    /// The localized string to use as the title of the action.
    #[property]
    pub fn title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), title]) }
    }

    /// The icon associated with the action.
    #[property]
    pub fn icon(&self) -> Option<UNNotificationActionIcon> {
        unsafe { to_optional(msg_send![self.m_self(), icon]) }
    }

    /* Getting Options
     */

    /// The behaviors associated with the action.
    #[property]
    pub fn options(&self) -> UNNotificationActionOptions {
        unsafe { msg_send![self.m_self(), options] }
    }
}
