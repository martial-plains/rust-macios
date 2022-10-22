#![allow(trivial_casts)]

use std::sync::Once;

use block::{ConcreteBlock, IntoConcreteBlock};
use objc::{
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl, Encode,
};

use crate::objective_c_runtime::{class, id, traits::PNSObject};

use super::{
    UNNotification, UNNotificationResponse, UNUserNotificationCenter,
    UNUSER_NOTIFICATION_CENTER_PTR,
};

#[repr(u64)]
#[derive(Debug)]
/// Constants indicating how to present a notification in a foreground app.
pub enum UNNotificationPresentationOptions {
    ///
    None = 0,
    /// Apply the notification's badge value to the app’s icon.
    Badge = (1 << 0),
    /// Play the sound associated with the notification.
    Sound = (1 << 1),
    /// Display the alert using the content provided by the notification.
    #[deprecated = "Use 'List | Banner' instead."]
    Alert = (1 << 2),
    /// Show the notification in Notification Center.
    List = (1 << 3),
    /// Present the notification as a banner.
    Banner = (1 << 4),
}

/// An interface for processing incoming notifications and responding to notification actions.
pub trait PUNUserNotificationCenterDelegate: PNSObject {
    /* Handling the Selection of Custom Actions
     */

    /// Asks the delegate to process the user's response to a delivered notification.
    fn user_notification_center_did_receive_notification_response_with_completion_handler<F>(
        &mut self,
        center: &UNUserNotificationCenter,
        response: &UNNotificationResponse,
        completion_handler: F,
    ) where
        F: IntoConcreteBlock<(), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(completion_handler);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), userNotificationCenter:center.m_self() didReceiveNotificationResponse:response.m_self() withCompletionHandler: block]
        }
    }

    /* Receiving Notifications
     */

    /// Asks the delegate how to handle a notification that arrived while the app was running in the foreground.
    fn user_notification_center_will_present_notification_with_completion_handler<F>(
        &mut self,
        center: &UNUserNotificationCenter,
        notification: &UNNotification,
        completion_handler: F,
    ) where
        F: IntoConcreteBlock<(UNNotificationPresentationOptions,), Ret = ()> + 'static,
    {
        let block = ConcreteBlock::new(completion_handler);
        let block = block.copy();

        unsafe {
            msg_send![self.m_self(), userNotificationCenter:center.m_self() willPresentNotification:notification.m_self() withCompletionHandler: block]
        }
    }

    /* Displaying Notification Settings
     */

    /// Asks the delegate to display the in-app notification settings.
    fn user_notification_center_open_settings_for_notification(
        &mut self,
        center: &UNUserNotificationCenter,
        notification: &UNNotification,
    ) {
        unsafe {
            msg_send![self.m_self(), userNotificationCenter: center.m_self() openSettingsForNotification: notification.m_self()]
        }
    }
}

/// A handy method for grabbing our [`UNUserNotificationCenterDelegate`] from the pointer. This is different from our
/// standard `utils` version as this doesn't require `RefCell` backing.
fn user_notification_center<T>(this: &mut Object) -> &mut T {
    unsafe {
        let app_ptr: usize = *this.get_ivar(UNUSER_NOTIFICATION_CENTER_PTR);
        let app = app_ptr as *mut T;
        &mut *app
    }
}

extern "C" fn user_notification_center_did_receive_notification_response_with_completion_handler<
    T: PUNUserNotificationCenterDelegate,
    F: Fn() + Send + Sync + 'static,
>(
    this: &mut Object,
    _: Sel,
    _: id,
    center: UNUserNotificationCenter,
    response: UNNotificationResponse,
    completion_handler: F,
) {
    user_notification_center::<T>(this)
        .user_notification_center_did_receive_notification_response_with_completion_handler(
            &center,
            &response,
            completion_handler,
        )
}

extern "C" fn user_notification_center_will_present_notification_with_completion_handler<
    T: PUNUserNotificationCenterDelegate,
    F: Fn(UNNotificationPresentationOptions) + Send + Sync + 'static,
>(
    this: &mut Object,
    _: Sel,
    _: id,
    center: UNUserNotificationCenter,
    notification: UNNotification,
    completion_handler: F,
) {
    user_notification_center::<T>(this)
        .user_notification_center_will_present_notification_with_completion_handler(
            &center,
            &notification,
            completion_handler,
        )
}

extern "C" fn user_notification_center_open_settings_for_notification<
    T: PUNUserNotificationCenterDelegate,
>(
    this: &mut Object,
    _: Sel,
    _: id,
    center: UNUserNotificationCenter,
    notification: UNNotification,
) {
    user_notification_center::<T>(this)
        .user_notification_center_open_settings_for_notification(&center, &notification)
}

/// Registers an `NSObject` application delegate, and configures it for the various callbacks and
/// pointers we need to have.
pub fn register_user_notification_center_delegate_class<
    T: PUNUserNotificationCenterDelegate + PUNUserNotificationCenterDelegate,
    F1: Fn() + Send + Sync + 'static + Encode,
    F2: Fn(UNNotificationPresentationOptions) + Send + Sync + 'static + Encode,
>() -> *const Class {
    static mut DELEGATE_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSObject);
        let mut decl = ClassDecl::new("RSTUNUserNotificationCenterDelegate", superclass).unwrap();

        decl.add_ivar::<usize>(UNUSER_NOTIFICATION_CENTER_PTR);

        decl.add_method(
            sel!(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:),
            user_notification_center_did_receive_notification_response_with_completion_handler::<
                T,
                F1,
            > as extern "C" fn(&mut Object, _, _, _, _, _),
        );

        decl.add_method(
            sel!(userNotificationCenter:willPresentNotification:withCompletionHandler:),
            user_notification_center_will_present_notification_with_completion_handler::<T, F2>
                as extern "C" fn(&mut Object, _, _, _, _, _),
        );

        decl.add_method(
            sel!(userNotificationCenter:openSettingsForNotification:),
            user_notification_center_open_settings_for_notification::<T>
                as extern "C" fn(&mut Object, _, _, _, _),
        );

        DELEGATE_CLASS = decl.register();
    });

    unsafe { DELEGATE_CLASS }
}
