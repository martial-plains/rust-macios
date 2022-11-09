use libc::c_float;
use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    object,
    objective_c_runtime::traits::{FromId, PNSObject},
};

use super::UNNotificationSoundName;

object! {
    /// The sound played upon delivery of a notification.
    unsafe pub struct UNNotificationSound;
}

#[interface_impl(NSObject)]
impl UNNotificationSound {
    /* Creating Notification Sounds
     */

    /// Returns an object representing the default sound for notifications.
    #[property]
    pub fn default_sound() -> UNNotificationSound {
        unsafe { UNNotificationSound::from_id(msg_send![Self::m_class(), defaultSound]) }
    }

    /// Creates a sound object that represents a custom sound file.
    #[method]
    pub fn sound_named(name: UNNotificationSoundName) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), soundNamed: name]) }
    }

    /* Getting Critical Sounds
     */

    /// The default sound used for critical alerts.
    #[property]
    pub fn default_critical_sound() -> UNNotificationSound {
        unsafe { UNNotificationSound::from_id(msg_send![Self::m_class(), defaultCriticalSound]) }
    }

    /// Creates a sound object that plays the default critical alert sound at the volume you specify.
    #[method]
    pub fn default_critical_sound_with_audio_volume(volume: c_float) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                defaultCriticalSoundWithAudioVolume: volume
            ])
        }
    }

    /// Creates a custom sound object for critical alerts.
    #[method]
    pub fn critical_sound_named(name: UNNotificationSoundName) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), criticalSoundNamed: name]) }
    }

    /// Creates a custom sound object for critical alerts with the volume you specify.
    #[method]
    pub fn critical_sound_named_with_audio_volume(
        name: UNNotificationSoundName,
        volume: c_float,
    ) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), criticalSoundNamed: name withAudioVolume: volume],
            )
        }
    }

    /* Type Properties
     */

    #[property]
    pub fn default_ringtone_sound() -> UNNotificationSound {
        unsafe { UNNotificationSound::from_id(msg_send![Self::m_class(), defaultRingtoneSound]) }
    }

    /* Type Methods
     */

    #[method]
    pub fn ringtone_sound_named(name: UNNotificationSoundName) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![Self::m_class(), ringtoneSoundNamed: name]) }
    }
}
