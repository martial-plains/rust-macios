use libc::c_double;
use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSDictionary, NSNumber, NSString, UInt},
    objective_c_runtime::{id, macros::object},
};

use super::{
    IUNNotificationContent, UNNotificationAttachment, UNNotificationInterruptionLevel,
    UNNotificationSound,
};

object! {
    /// The editable content for a notification.
    unsafe pub struct UNMutableNotificationContent;
}

impl IUNNotificationContent for UNMutableNotificationContent {}

#[interface_impl(UNNotificationContent)]
impl UNMutableNotificationContent {
    /* Providing the primary content
     */
    /// The localized text that provides the notification’s primary description.
    #[property]
    pub fn set_title(&mut self, title: NSString) {
        unsafe { msg_send![self.m_self(), setTitle: title] }
    }

    /// The localized text that provides the notification’s secondary description.
    #[property]
    pub fn set_subtitle(&mut self, subtitle: NSString) {
        unsafe { msg_send![self.m_self(), setSubtitle: subtitle] }
    }

    /// The localized text that provides the notification’s main content.
    #[property]
    pub fn set_body(&mut self, body: NSString) {
        unsafe { msg_send![self.m_self(), setBody: body] }
    }

    /* Providing supplementary content
     */

    /// The visual and audio attachments to display alongside the notification’s main content.
    #[property]
    pub fn set_attachments(&mut self, attachments: NSArray<UNNotificationAttachment>) {
        unsafe { msg_send![self.m_self(), setAttachments: attachments] }
    }

    /// The custom data to associate with the notification.
    #[property]
    pub fn set_user_info(&mut self, user_info: NSDictionary<id, id>) {
        unsafe { msg_send![self.m_self(), setUserInfo: user_info] }
    }

    /* Configuring app behavior
     */

    /// The name of the image or storyboard to use when your app launches because of the notification.
    #[property]
    pub fn set_launch_image_name(&mut self, value: NSString) {
        unsafe { msg_send![self.m_self(), setLaunchImageName: value] }
    }

    /// The number that your app’s icon displays.
    #[property]
    pub fn set_badge(&mut self, badge: NSNumber) {
        unsafe { msg_send![self.m_self(), setBadge: badge] }
    }

    /// The value your app uses to determine which scene to display to handle the notification.
    #[property]
    pub fn set_target_content_identifier(&mut self, value: NSString) {
        unsafe { msg_send![self.m_self(), setTargetContentIdentifier: value] }
    }

    /* Integrating with the system
     */

    /// The sound that plays when the system delivers the notification.
    #[property]
    pub fn set_sound(&mut self, value: UNNotificationSound) {
        unsafe { msg_send![self.m_self(), setSound: value] }
    }

    /// The notification’s importance and required delivery timing.
    #[property]
    pub fn set_interruption_level(&mut self, value: UNNotificationInterruptionLevel) {
        unsafe { msg_send![self.m_self(), setInterruptionLevel: value] }
    }

    ///The score the system uses to determine if the notification is the summary’s featured notification.
    #[property]
    pub fn set_relevance_score(&mut self, value: c_double) {
        unsafe { msg_send![self.m_self(), setRelevanceScore: value] }
    }

    /// The criteria the system evaluates to determine if it displays the notification in the current Focus.
    #[property]
    pub fn set_filter_criteria(&mut self, value: NSString) {
        unsafe { msg_send![self.m_self(), setFilterCriteria: value] }
    }

    /* Grouping notifications
     */

    /// The identifier that groups related notifications.
    #[property]
    pub fn set_thread_identifier(&mut self, value: NSString) {
        unsafe { msg_send![self.m_self(), setThreadIdentifier: value] }
    }

    /// The identifier of the notification’s category.
    #[property]
    pub fn set_category_identifier(&mut self, value: NSString) {
        unsafe { msg_send![self.m_self(), setCategoryIdentifier: value] }
    }

    /// The text the system adds to the notification summary to provide additional context.
    #[property]
    pub fn set_summary_argument(&mut self, value: NSString) {
        unsafe { msg_send![self.m_self(), setSummaryArgument: value] }
    }

    /// The number the system adds to the notification summary when the notification represents multiple items.
    #[property]
    pub fn set_summary_argument_count(&mut self, value: UInt) {
        unsafe { msg_send![self.m_self(), setSummaryArgumentCount: value] }
    }
}
