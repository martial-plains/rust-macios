use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
    },
};

use super::{UNNotificationContent, UNNotificationTrigger};

object! {
    /// A request to schedule a local notification, which includes the content of the notification and the trigger conditions for delivery.
    unsafe pub struct UNNotificationRequest;
}

#[interface_impl(NSObject)]
impl UNNotificationRequest {
    /* Creating a Notification Request
     */

    /// Creates a notification request object that you use to schedule a notification.
    #[method]
    pub fn request_with_identifier_content_trigger(
        identifier: &NSString,
        content: &UNNotificationContent,
        trigger: &UNNotificationTrigger,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(),  requestWithIdentifier: identifier.m_self() content: content.m_self() trigger: trigger.m_self()],
            )
        }
    }

    /* Getting the Request Details
     */

    /// The unique identifier for this notification request.
    #[property]
    pub fn identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }

    /// The content associated with the notification.
    #[property]
    pub fn content(&self) -> UNNotificationContent {
        unsafe { UNNotificationContent::from_id(msg_send![self.m_self(), content]) }
    }

    /// The conditions that trigger the delivery of the notification.
    #[property]
    pub fn trigger(&self) -> UNNotificationTrigger {
        unsafe { UNNotificationTrigger::from_id(msg_send![self.m_self(), trigger]) }
    }
}
