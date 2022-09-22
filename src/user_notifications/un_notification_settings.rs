use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    objective_c_runtime::{macros::object, traits::PNSObject},
    utils::to_bool,
};

object! {
    /// The object for managing notification-related settings and the authorization status of your app.
    unsafe pub struct UNNotificationSettings;
}

/// Constants indicating whether the app is allowed to schedule notifications.
#[repr(i64)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum UNAuthorizationStatus {
    /// The user hasn't yet made a choice about whether the app is allowed to schedule notifications.
    NotDetermined = 0,
    /// The app isn't authorized to schedule or receive notifications.
    Denied,
    /// The app is authorized to schedule or receive notifications.
    Authorized,
    /// The application is provisionally authorized to post noninterruptive user notifications.
    #[cfg(target_os = "ios")]
    Provisional,
    /// The app is authorized to schedule or receive notifications for a limited amount of time.
    #[cfg(target_os = "ios")]
    Ephemeral,
}

/// Constants that indicate the current status of a notification setting.
#[repr(i64)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum UNNotificationSetting {
    /// The setting is not available to your app.
    NotSupported = 0,
    /// The setting is disabled.
    Disabled,
    /// The setting is enabled.
    Enabled,
}

/// Constants indicating the presentation styles for alerts.
#[repr(i64)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum UNAlertStyle {
    /// No alert.
    None = 0,
    /// Banner alerts.
    Banner,
    /// Modal alerts.
    Alert,
}

/// Constants indicating the style previewing a notification's content.
#[repr(i64)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum UNShowPreviewsSetting {
    /// The notification's content is always shown, even when the device is locked.
    Always,
    /// The notification's content is shown only when the device is unlocked.
    WhenAuthenticated,
    /// The notification's content is never shown, even when the device is unlocked
    Never,
}

#[interface_impl(NSObject)]
impl UNNotificationSettings {
    /* Getting the Authorization Status
     */

    /// The app's ability to schedule and receive local and remote notifications.
    #[property]
    pub fn authorization_status(&self) -> UNAuthorizationStatus {
        unsafe { msg_send![self.m_self(), authorizationStatus] }
    }

    /* Getting Device-Specific Settings
     */

    /// The setting that indicates whether your app’s notifications appear in Notification Center.
    #[property]
    pub fn notification_center_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), notificationCenterSetting] }
    }

    /// The setting that indicates whether your app’s notifications appear on a device’s Lock screen.
    #[property]
    pub fn lock_screen_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), lockScreenSetting] }
    }

    /// The setting that indicates whether your app’s notifications appear in CarPlay.
    #[property]
    pub fn car_play_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), carPlaySetting] }
    }

    /// The authorization status for displaying alerts.
    #[property]
    pub fn alert_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), alertSetting] }
    }

    /// The setting that indicates whether badges appear on your app’s icon.
    #[property]
    pub fn badge_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), badgeSetting] }
    }

    /// The authorization status for playing sounds for incoming notifications.
    #[property]
    pub fn sound_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), soundSetting] }
    }

    /// The authorization status for playing sounds for critical alerts.
    #[property]
    pub fn critical_alert_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), criticalAlertSetting] }
    }

    /// The setting that indicates whether Siri can announce your app’s notifications.
    #[property]
    pub fn announcement_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), announcementSetting] }
    }

    /// The setting that indicates the system schedules the notification.
    #[property]
    pub fn scheduled_delivery_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), scheduledDeliverySetting] }
    }

    /// The setting that indicates the system treats the notification as time-sensitive.
    #[property]
    pub fn time_sensitive_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), timeSensitiveSetting] }
    }

    /* Getting Interface Settings
     */

    /// The setting that indicates whether the app shows a preview of the notification's content.
    #[property]
    pub fn show_previews_setting(&self) -> UNShowPreviewsSetting {
        unsafe { msg_send![self.m_self(), showPreviewsSetting] }
    }

    /// A Boolean value indicating the system displays a button for in-app notification settings.
    #[property]
    pub fn provides_app_notification_settings(&self) -> bool {
        unsafe { to_bool(msg_send![self.m_self(), providesAppNotificationSettings]) }
    }

    ///
    #[property]
    pub fn direct_messages_setting(&self) -> UNNotificationSetting {
        unsafe { msg_send![self.m_self(), directMessagesSetting] }
    }
}
