use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSDictionary, NSError, NSString, NSURL},
    objective_c_runtime::{
        id,
        macros::object,
        traits::{FromId, PNSObject},
    },
};

object! {
    /// A media file associated with a notification.
    unsafe pub struct UNNotificationAttachment;
}

extern "C" {
    /// A hint about an attachment’s file type.
    pub static UNNotificationAttachmentOptionsTypeHintKey: NSString;

    /// A Boolean value indicating whether the system hides the attachment’s thumbnail.
    pub static UNNotificationAttachmentOptionsThumbnailHiddenKey: NSString;

    /// The clipping rectangle for a thumbnail image.
    pub static UNNotificationAttachmentOptionsThumbnailClippingRectKey: NSString;

    /// The frame number of an animation to use as a thumbnail image.
    pub static UNNotificationAttachmentOptionsThumbnailTimeKey: NSString;
}

#[interface_impl(NSObject)]
impl UNNotificationAttachment {
    /* Creating an Attachment
     */

    /// Creates an attachment object from the specified file and options.
    #[method]
    pub fn attachment_with_identifier_url_options(
        identifier: NSString,
        url: NSURL,
        options: NSDictionary<id, id>,
    ) -> Result<Self, NSError>
    where
        Self: Sized + FromId,
    {
        unsafe {
            let mut error = NSError::m_alloc();
            let ptr = Self::from_id(msg_send![
                Self::m_class(),
                attachmentWithIdentifier: identifier
                URL: url
                options: options
                error: &mut error
            ]);

            if error.m_self().is_null() {
                Ok(ptr)
            } else {
                Err(error)
            }
        }
    }

    /* Getting the Attachment Contents
     */

    /// The unique identifier for the attachment.
    #[property]
    pub fn identifier(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), identifier]) }
    }

    /// The URL of the file for this attachment.
    #[property]
    pub fn url(&self) -> NSURL {
        unsafe { NSURL::from_id(msg_send![self.m_self(), URL]) }
    }

    /// The UTI type of the attachment.
    #[property]
    pub fn _type(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![self.m_self(), type]) }
    }
}
