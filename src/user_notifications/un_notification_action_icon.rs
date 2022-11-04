use objc::{msg_send, sel, sel_impl};
use rust_macios_objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::NSString,
    objective_c_runtime::{
        macros::object,
        traits::{FromId, PNSObject},
    },
};

object! {
    unsafe pub struct UNNotificationActionIcon;
}

#[interface_impl(NSObject)]
impl UNNotificationActionIcon {
    /* Essentials
     */

    /// Creates an action icon by using a system symbol image.
    #[method]
    pub fn icon_with_system_image_name(system_image_name: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                iconWithSystemImageName: system_image_name
            ])
        }
    }

    /// Creates an action icon based on an image in your app’s bundle, preferably in an asset catalog.
    #[method]
    pub fn icon_with_template_image_name(template_image_name: NSString) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(msg_send![
                Self::m_class(),
                iconWithTemplateImageName: template_image_name
            ])
        }
    }
}
