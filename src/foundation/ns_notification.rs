use objc::{msg_send, sel, sel_impl, Encode};
use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{
    id,
    macros::object,
    traits::{FromId, PNSObject},
};

use super::{NSCoder, NSDictionary, NSNotificationName};

object! {
    /// A container for information broadcast through a notification center to all registered observers.
    unsafe pub struct NSNotification;
}

unsafe impl Encode for NSNotification {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("@") }
    }
}

#[interface_impl(NSObject)]
impl NSNotification {
    /* Creating Notifications
     */

    /// Initializes an empty notification.
    #[method]
    pub fn init(&mut self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let ptr = msg_send![self.m_self(), init];
            FromId::from_id(ptr)
        }
    }

    /// Initializes a notification with the data from an unarchiver.
    #[method]
    pub fn init_with_coder(&mut self, coder: &NSCoder) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { FromId::from_id(msg_send![self.m_self(), initWithCoder: coder.m_self()]) }
    }

    /// Returns a new notification object with a specified name and object.
    #[method]
    pub fn notification_with_name_object(name: NSNotificationName, object: id) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            FromId::from_id(msg_send![Self::m_class(), notificationWithName: name object: object])
        }
    }

    /// Returns a notification object with a specified name, object, and user information.
    #[method]
    pub fn notification_with_name_object_user_info(
        name: NSNotificationName,
        object: id,
        user_info: NSDictionary<id, id>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            FromId::from_id(
                msg_send![Self::m_class(), notificationWithName: name object: object userInfo: user_info],
            )
        }
    }

    /// Initializes a notification with a specified name, object, and user information.
    #[method]
    pub fn init_with_name_object_user_info(
        &mut self,
        name: &NSNotificationName,
        object: id,
        user_info: &NSDictionary<id, id>,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            FromId::from_id(
                msg_send![self.m_self(), initWithName: name.m_self() object: object userInfo: user_info.m_self()],
            )
        }
    }

    /* Getting Notification Information
     */

    /// The name of the notification.
    #[method]
    pub fn name(&self) -> NSNotificationName {
        unsafe { NSNotificationName::from_id(msg_send![self.m_self(), name]) }
    }

    /// The object associated with the notification.
    #[method]
    pub fn object(&self) -> id {
        unsafe { msg_send![self.m_self(), object] }
    }

    /// The user information dictionary associated with the notification.
    #[method]
    pub fn user_info(&self) -> NSDictionary<id, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), userInfo]) }
    }
}
