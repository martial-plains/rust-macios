use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSString},
    objective_c_runtime::{
        macros::object,
        nil,
        traits::{FromId, PNSObject},
    },
};

use super::un_notification_action::UNNotificationAction;

object! {
    /// A type of notification your app supports and the custom actions that the system displays.
    unsafe pub struct UNNotificationCategory;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u64)]
pub enum UNNotificationCategoryOptions {
    None = 0,
    CustomDismissAction = (1 << 0),
    AllowInCarPlay = (1 << 1),
    HiddenPreviewsShowTitle = (1 << 2),
    HiddenPreviewsShowSubtitle = (1 << 3),
    #[deprecated]
    AllowAnnouncement = (1 << 4),
}

#[interface_impl(NSObject)]
impl UNNotificationCategory {
    /* Essentials
     */

    /// Creates a category object containing the specified actions and options.
    #[method]
    pub fn category_with_identifier_actions_intent_identifiers_options(
        identifier: NSString,
        actions: NSArray<UNNotificationAction>,
        intent_identifiers: NSArray<NSString>,
        options: &[UNNotificationCategoryOptions],
    ) -> Self
    where
        Self: Sized + FromId,
    {
        let options = options
            .iter()
            .fold(0u64, |init, option| init | *option as u64);

        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), categoryWithIdentifier: identifier actions: actions intentIdentifiers: intent_identifiers options: options],
            )
        }
    }

    /// Creates a category object containing the specified actions, options, and placeholder text used when previews aren’t shown.
    #[method]
    pub fn category_with_identifier_actions_intent_identifiers_hidden_previews_body_placeholder_options(
        identifier: NSString,
        actions: NSArray<UNNotificationAction>,
        intent_identifiers: NSArray<NSString>,
        hidden_previews_body_placeholder: Option<NSString>,
        options: &[UNNotificationCategoryOptions],
    ) -> Self
    where
        Self: Sized + FromId,
    {
        let hidden_previews_body_placeholder = match hidden_previews_body_placeholder {
            Some(value) => value.m_self(),
            None => nil,
        };

        let options = options
            .iter()
            .fold(0u64, |init, option| init | *option as u64);

        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), categoryWithIdentifier: identifier actions: actions intentIdentifiers: intent_identifiers hiddenPreviewsBodyPlaceholder: hidden_previews_body_placeholder options: options],
            )
        }
    }

    pub fn categoryWithIdentifier_actions_intentIdentifiers_hiddenPreviewsBodyPlaceholder_categorySummaryFormat_options(
        identifier: NSString,
        actions: NSArray<UNNotificationAction>,
        intent_identifiers: NSArray<NSString>,
        hidden_previews_body_placeholder: NSString,
        category_summary_format: NSString,
        options: &[UNNotificationCategoryOptions],
    ) -> Self
    where
        Self: Sized + FromId,
    {
        let options = options
            .iter()
            .fold(0u64, |init, option| init | *option as u64);

        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), categoryWithIdentifier: identifier actions: actions intentIdentifiers: intent_identifiers hiddenPreviewsBodyPlaceholder: hidden_previews_body_placeholder categorySummaryFormat: categorySummaryFormat options: options],
            )
        }
    }

    /* Getting the Information
     */

    /// The unique string assigned to the category.
    #[property]
    pub fn identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }

    /// The actions to display when the system delivers notifications of this type.
    #[property]
    pub fn actions(&self) -> NSArray<UNNotificationAction> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), actions]) }
    }

    /// The intents related to notifications of this category.
    #[property]
    pub fn intent_identifiers(&self) -> NSArray<NSString> {
        unsafe { NSArray::from_id(msg_send![self.m_self(), intentIdentifiers]) }
    }

    /// The placeholder text to display when the system disables notification previews for the app.
    #[property]
    pub fn hidden_previews_body_placeholder(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), hiddenPreviewsBodyPlaceholder]) }
    }

    /// A format string for the summary description used when the system groups the category’s notifications.
    #[property]
    pub fn category_summary_format(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), categorySummaryFormat]) }
    }

    /// Options for how to handle notifications of this type.
    #[property]
    pub fn options(&self) -> UNNotificationCategoryOptions {
        unsafe { msg_send![self.m_self(), options] }
    }
}
