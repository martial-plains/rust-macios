use block::IntoConcreteBlock;
use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{macros::object, traits::PNSObject};

use super::{UNNotificationContent, UNNotificationRequest};

object! {
    /// An object that modifies the content of a remote notification before it’s delivered to the user.
    unsafe pub struct UNNotificationServiceExtension;
}

#[interface_impl(NSObject)]
impl UNNotificationServiceExtension {
    /// Asks you to make any needed changes to the notification and notify the system when you’re done.
    #[method]
    fn did_receive_notification_request_with_content_handler<F>(
        &self,
        request: UNNotificationRequest,
        content_handler: F,
    ) where
        F: IntoConcreteBlock<(UNNotificationContent,)> + 'static,
    {
        unsafe {
            let block = content_handler.into_concrete_block();
            let block = block.copy();
            msg_send![self.m_self(), didReceiveNotificationRequest: request withContentHandler: block]
        }
    }

    /// Tells you that the system is terminating your extension.
    #[method]
    fn service_extension_time_will_expire(&self) {
        unsafe { msg_send![self.m_self(), serviceExtensionTimeWillExpire] }
    }
}
