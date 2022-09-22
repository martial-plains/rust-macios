use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSDate,
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
    },
};

use super::UNNotificationRequest;

object! {
    /// The data for a local or remote notification the system delivers to your app.
    unsafe pub struct UNNotification;
}

#[interface_impl(NSObject)]
impl UNNotification {
    /* Getting the Notification Details
     */

    /// The notification request containing the payload and trigger condition for the notification.
    #[property]
    pub fn request(&self) -> UNNotificationRequest {
        unsafe { UNNotificationRequest::from_id(msg_send![self.m_self(), request]) }
    }

    /// The delivery date of the notification.
    #[property]
    pub fn date(&self) -> NSDate {
        unsafe { NSDate::from_id(msg_send![self.m_self(), date]) }
    }
}
