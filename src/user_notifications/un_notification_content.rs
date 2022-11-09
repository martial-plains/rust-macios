use libc::c_double;
use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSDictionary, NSError, NSNumber, NSString, UInt},
    object,
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject},
    },
    utils::to_optional,
};

use super::{UNNotificationAttachment, UNNotificationSound};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i64)]
pub enum UNNotificationInterruptionLevel {
    Passive,
    Active,
    TimeSensitive,
    Critical,
}

object! {
    /// The uneditable content of a notification.
    unsafe pub struct UNNotificationContent;
}

#[interface_impl(NSObject)]
impl UNNotificationContent {
    /// The localized text that provides the notification’s primary description.
    #[property]
    pub fn title(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), title]) }
    }

    /// The localized text that provides the notification’s secondary description.
    #[property]
    pub fn subtitle(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), subtitle]) }
    }

    /// The localized text that provides the notification’s main content.
    #[property]
    pub fn body(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), body]) }
    }

    /* Accessing supplementary content
     */

    /// The visual and audio attachments to display alongside the notification’s main content.
    #[property]
    pub fn attachments(&self) -> NSArray<UNNotificationAttachment> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), attachments]) }
    }

    /// The custom data to associate with the notification.
    #[property]
    pub fn user_info(&self) -> NSDictionary<id, id> {
        unsafe { NSDictionary::from_id(msg_send![self.m_self(), userInfo]) }
    }

    /* Reading app configuration
     */

    /// The name of the image or storyboard to use when your app launches because of the notification.
    #[property]
    pub fn launch_image_name(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), launchImageName]) }
    }

    /// The number that your app’s icon displays.
    #[property]
    pub fn badge(&self) -> Option<NSNumber> {
        unsafe { to_optional(msg_send![self.m_self(), badge]) }
    }

    /// The value your app uses to determine which scene to display to handle the notification.
    #[property]
    pub fn target_content_identifier(&self) -> Option<NSString> {
        unsafe { to_optional(msg_send![self.m_self(), targetContentIdentifier]) }
    }

    /* Reading system configuration
     */

    /// The sound that plays when the system delivers the notification.
    #[property]
    pub fn sound(&self) -> Option<UNNotificationSound> {
        unsafe { to_optional(msg_send![self.m_self(), sound]) }
    }

    /// The notification’s importance and required delivery timing.
    #[property]
    pub fn interruption_level(&self) -> UNNotificationInterruptionLevel {
        unsafe { msg_send![self.m_self(), interruptionLevel] }
    }

    ///The score the system uses to determine if the notification is the summary’s featured notification.
    #[property]
    pub fn relevance_score(&self) -> c_double {
        unsafe { msg_send![self.m_self(), relevanceScore] }
    }

    /// The criteria the system evaluates to determine if it displays the notification in the current Focus.
    #[property]
    pub fn filter_criteria(&self) -> Option<NSString> {
        unsafe { to_optional(msg_send![self.m_self(), filterCriteria]) }
    }

    /* Retrieving group information
     */

    /// The identifier that groups related notifications.
    #[property]
    pub fn thread_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), threadIdentifier]) }
    }

    /// The identifier of the notification’s category.
    #[property]
    pub fn category_identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), categoryIdentifier]) }
    }

    /// The text the system adds to the notification summary to provide additional context.
    #[property]
    pub fn summary_argument(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), summaryArgument]) }
    }

    /// The number the system adds to the notification summary when the notification represents multiple items.
    #[property]
    pub fn summary_argument_count(&self) -> UInt {
        unsafe { msg_send![self.m_self(), summaryArgumentCount] }
    }

    /* Updating the notification’s content
     */

    /// Returns a copy of the notification that includes content from the specified provider.
    #[method]
    pub fn content_by_updating_with_provider(
        &mut self,
        provider: id,
    ) -> Result<UNNotificationContent, NSError> {
        unsafe {
            let mut error = NSError::m_alloc();

            let ptr = UNNotificationContent::from_id(
                msg_send![self.m_self(), contentByUpdatingWithProvider: provider error: &mut error],
            );

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }
}
