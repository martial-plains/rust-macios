use rust_macios::user_notifications::{UNAuthorizationOptions, UNUserNotificationCenter};

fn main() {
    let mut center = UNUserNotificationCenter::current_notification_center();

    center.request_authorization_with_options_completion_handler(
        UNAuthorizationOptions::Sound,
        |granted, error| {},
    )
}
