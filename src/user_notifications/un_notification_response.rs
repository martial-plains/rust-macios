use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSString,
    object,
    objective_c_runtime::traits::{FromId, PNSObject},
    uikit::UIScene,
    utils::to_optional,
};

use super::UNNotification;

object! {
    /// The user’s response to an actionable notification.
    unsafe pub struct UNNotificationResponse;
}

#[interface_impl(NSObject)]
impl UNNotificationResponse {
    /// The identifier string of the action that the user selected.
    #[property]
    pub fn action_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), actionIdentifier]) }
    }

    /// The notification to which the user responded.
    #[property]
    pub fn notification(&self) -> UNNotification {
        unsafe { UNNotification::from_id(msg_send![self.m_self(), notification]) }
    }

    /// The scene where the system reflects the user’s response to a notification.
    ///
    ///
    /// Required features: `"uikit"`
    #[property]
    pub fn target_scene(&self) -> Option<UIScene> {
        unsafe { to_optional(msg_send![self.m_self(), targetScene]) }
    }
}
