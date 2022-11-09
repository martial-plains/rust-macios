use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::object;

use super::IUNNotificationTrigger;

object! {
    /// A trigger condition that indicates Apple Push Notification Service (APNs) has sent the notification.
    unsafe pub struct UNPushNotificationTrigger;
}

impl IUNNotificationTrigger for UNPushNotificationTrigger {}

#[interface_impl(UNNotificationTrigger)]
impl UNPushNotificationTrigger {}
