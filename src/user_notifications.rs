//! Push user-facing notifications to the user’s device from a server, or generate them locally from your app.

use crate::foundation::NSString;

/* Notification Management
 */

mod un_user_notification_center;
pub use un_user_notification_center::*;

mod un_user_notification_center_delegate;
pub use un_user_notification_center_delegate::*;

mod un_notification_settings;
pub use un_notification_settings::*;

/* Notification Requests
*/

mod un_notification_request;
pub use un_notification_request::*;

mod un_notification;
pub use un_notification::*;

/* Notification Content
 */

mod un_notification_content_providing;
pub use un_notification_content_providing::*;

mod un_notification_action_icon;
pub use un_notification_action_icon::*;

mod un_mutable_notification_content;
pub use un_mutable_notification_content::*;

mod un_notification_content;
pub use un_notification_content::*;

mod un_notification_attachment;
pub use un_notification_attachment::*;

mod un_notification_sound;
pub use un_notification_sound::*;

pub type UNNotificationSoundName = NSString;

/* Triggers
 */

mod un_calendar_notification_trigger;
pub use un_calendar_notification_trigger::*;

mod un_time_interval_notification_trigger;
pub use un_time_interval_notification_trigger::*;

mod un_location_notification_trigger;
pub use un_location_notification_trigger::*;

mod un_push_notification_trigger;
pub use un_push_notification_trigger::*;

mod un_notification_trigger;
pub use un_notification_trigger::*;

/* Notification Categories and User Actions
 */

mod un_notification_category;
pub use un_notification_category::*;

mod un_notification_action;
pub use un_notification_action::*;

mod un_text_input_notification_action;
pub use un_text_input_notification_action::*;

/* Notification Responses
 */

mod un_notification_response;
pub use un_notification_response::*;

mod un_text_input_notification_response;
pub use un_text_input_notification_response::*;

/* Notification Service App Extension
 */

mod un_notification_service_extension;
pub use un_notification_service_extension::*;
