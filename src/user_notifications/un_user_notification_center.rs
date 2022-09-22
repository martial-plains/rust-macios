use block::{ConcreteBlock, IntoConcreteBlock};
use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{Int, NSArray, NSError, NSSet, NSString},
    objective_c_runtime::{
        id,
        macros::object,
        traits::{FromId, PNSObject},
    },
    utils::to_bool,
};

use super::{
    UNNotification, UNNotificationCategory, UNNotificationRequest, UNNotificationSettings,
};

object! {
    /// The central object for managing notification-related activities for your app or app extension.
    unsafe pub struct UNUserNotificationCenter;
}

/// Pointer for [`UNUserNotificationCenter`]
pub static UNUSER_NOTIFICATION_CENTER_PTR: &str = "rstUNUserNotificationCenterPtr";

/// Options that determine the authorized features of local and remote notifications.
#[repr(u64)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum UNAuthorizationOptions {
    /// No authorization options.
    None = 0,
    /// The ability to update the app’s badge.
    Badge = (1 << 0),
    /// The ability to play sounds.
    Sound = (1 << 1),
    /// The ability to display alerts.
    Alert = (1 << 2),
    /// The ability to display notifications in a CarPlay environment.
    CarPlay = (1 << 3),
    /// The ability to play sounds for critical alerts.
    #[cfg(target_os = "ios")]
    CriticalAlert = (1 << 4),
    /// An option indicating the system should display a button for in-app notification settings.
    #[cfg(target_os = "ios")]
    ProvidesAppNotificationSettings = (1 << 5),
    /// The ability to post noninterrupting notifications provisionally to the Notification Center.
    #[cfg(target_os = "ios")]
    Provisional = (1 << 6),
    /// The ability for Siri to automatically read out messages over AirPods.
    #[deprecated]
    #[cfg(target_os = "ios")]
    Announcement = (1 << 7),
    ///
    #[deprecated]
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    TimeSensitive = (1 << 8),
}

/// Constants that identify notification errors.
#[repr(i64)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum UNErrorCode {
    /// Notifications aren’t allowed.
    NotificationsNotAllowed = 1,
    /// The URL for an attachment was invalid.
    AttachmentInvalidUrl = 100,
    /// The file type of an attachment isn’t supported.
    AttachmentUnrecognizedType,
    /// An attachment is too large.
    AttachmentInvalidFileSize,
    /// The specified attachment isn’t in the system data store.
    AttachmentNotInDataStore,
    /// An error occurred when trying to move an attachment to the system data store.
    AttachmentMoveIntoDataStoreFailed,
    /// The file for an attachment is corrupt.
    AttachmentCorrupt,
    /// The notification doesn’t have an associated date, but should.
    NotificationInvalidNoDate = 1400,
    /// The notification has no user-facing content, but should.
    NotificationInvalidNoContent,
    ///
    ContentProvidingObjectNotAllowed = 1500,
    ///
    ContentProvidingInvalid = 1501,
}

#[interface_impl(NSObject)]
impl UNUserNotificationCenter {
    /* Managing the notification center
     */

    /// Returns your app’s notification center.
    #[method]
    pub fn current_notification_center() -> UNUserNotificationCenter
    where
        Self: Sized + FromId,
    {
        unsafe { msg_send![Self::m_class(), currentNotificationCenter] }
    }

    /// Retrieves the authorization and feature-related settings for your app.
    #[method]
    pub fn get_notification_settings_with_completion_handler<F>(&self, completion_handler: F)
    where
        F: IntoConcreteBlock<(UNNotificationSettings,), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(completion_handler);
        let block = block.copy();

        unsafe {
            msg_send![
                self.m_self(),
                getNotificationSettingsWithCompletionHandler: block
            ]
        }
    }

    /// Updates the badge count for your app’s icon.
    #[method]
    pub fn set_badge_count_with_completion_handler<F>(
        &mut self,
        new_badge_count: Int,
        completion_handler: F,
    ) where
        F: IntoConcreteBlock<(NSError,), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(completion_handler);
        let block = block.copy();
        unsafe {
            msg_send![self.m_self(), setBadgeCount: new_badge_count withCompletionHandler: block]
        }
    }

    /* Requesting authorization
     */

    /// Requests the user’s authorization to allow local and remote notifications for your app.
    #[method]
    pub fn request_authorization_with_options_completion_handler<F>(
        &mut self,
        options: UNAuthorizationOptions,
        completion_handler: F,
    ) where
        F: IntoConcreteBlock<(bool, NSError), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(completion_handler);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), requestAuthorizationWithOptions:options completionHandler: block]
        }
    }

    /* Processing received notifications
     */

    /// The delegate of the notification center.
    #[property]
    pub fn delegate(&self) -> id {
        unsafe { msg_send![self.m_self(), delegate] }
    }

    /// Sets the delegate of the notification center.
    ///
    /// # Arguments
    ///
    /// * `delegate` - The delegate to use.
    #[property]
    pub fn set_delegate(&self, delegate: id) {
        unsafe { msg_send![self.m_self(), setDelegate: delegate] }
    }

    /// A Boolean value that indicates whether the device supports notification content extensions.
    #[property]
    pub fn supports_content_extensions(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), supportsContentExtensions]) }
    }

    /* Scheduling notifications
     */

    /// Schedules the delivery of a local notification.
    #[method]
    pub fn add_notification_request_with_completion_handler<F>(
        &mut self,
        request: &UNNotificationRequest,
        completion_handler: F,
    ) where
        F: IntoConcreteBlock<(NSError,), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(completion_handler);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), addNotificationRequest: request.m_self() withCompletionHandler: block]
        }
    }

    /// Fetches all of your app’s local notifications that are pending delivery.
    #[method]
    pub fn get_pending_notification_requests_with_completion_handler<F>(
        &self,
        completion_handler: F,
    ) where
        F: IntoConcreteBlock<(NSArray<UNNotificationRequest>,)>,
    {
        unsafe {
            msg_send![
                self.m_self(),
                getPendingNotificationRequestsWithCompletionHandler: completion_handler
            ]
        }
    }

    /// Removes your app’s local notifications that are pending and match the specified identifiers.
    #[method]
    pub fn remove_pending_notification_requests_with_identifiers(
        &mut self,
        identifiers: &NSArray<NSString>,
    ) {
        unsafe {
            msg_send![
                self.m_self(),
                removePendingNotificationRequestsWithIdentifiers: identifiers.m_self()
            ]
        }
    }

    /// Removes all of your app’s pending local notifications.
    #[method]
    pub fn remove_all_pending_notification_requests(&mut self) {
        unsafe { msg_send![self.m_self(), removeAllPendingNotificationRequests] }
    }

    /*  Removing delivered notifications*/

    /// Fetches all of your app’s delivered notifications that are still present in Notification Center.
    #[method]
    pub fn get_delivered_notifications_with_completion_handler<F>(&self, completion_handler: F)
    where
        F: IntoConcreteBlock<(NSArray<UNNotification>,)>,
    {
        unsafe {
            msg_send![
                self.m_self(),
                getDeliveredNotificationsWithCompletionHandler: completion_handler
            ]
        }
    }

    /// Removes your app’s notifications from Notification Center that match the specified identifiers.
    #[method]
    pub fn remove_delivered_notifications_with_identifiers(
        &mut self,
        identifiers: &NSArray<NSString>,
    ) {
        unsafe {
            msg_send![
                self.m_self(),
                removeDeliveredNotificationsWithIdentifiers: identifiers.m_self()
            ]
        }
    }

    /// Removes all of your app’s delivered notifications from Notification Center.
    #[method]
    pub fn remove_all_delivered_notifications(&mut self) {
        unsafe { msg_send![self.m_self(), removeAllDeliveredNotifications] }
    }

    /*  Managing notification categories */

    /// Registers the notification categories that your app supports.
    #[method]
    pub fn set_notification_categories(&mut self, categories: &NSSet<UNNotificationCategory>) {
        unsafe { msg_send![self.m_self(), setNotificationCategories: categories.m_self()] }
    }

    /// Fetches your app’s registered notification categories.
    #[method]
    pub fn get_notification_categories_with_completion_handler<F>(&self, completion_handler: F)
    where
        F: IntoConcreteBlock<(NSSet<UNNotificationCategory>,)> + 'static,
    {
        unsafe {
            msg_send![
                self.m_self(),
                getNotificationCategoriesWithCompletionHandler: completion_handler
            ]
        }
    }
}
