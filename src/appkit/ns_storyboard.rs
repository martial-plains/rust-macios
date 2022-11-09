use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSBundle,
    object,
    objective_c_runtime::{
        id, nil,
        traits::{FromId, PNSObject},
    },
};

use super::NSStoryboardName;

object! {
    /// An encapsulation of the design-time view controller and window controller graph represented in an Interface Builder storyboard resource file.
    unsafe pub struct NSStoryboard
}

impl INSStoryboard for NSStoryboard {}

/// An encapsulation of the design-time view controller and window controller graph represented in an Interface Builder storyboard resource file.
pub trait INSStoryboard: PNSObject {
    /// Creates a storyboard based on the named storyboard file in the specified bundle.
    fn tm_storyboard_with_name_bundle(name: NSStoryboardName, bundle: Option<NSBundle>) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            Self::from_id(
                msg_send![Self::m_class(), storyboardWithName: name bundle: match bundle {
                    Some(val) => val.m_self(),
                    None => nil
                }],
            )
        }
    }

    /// The app's main storyboard.
    fn tp_main_storyboard() -> NSStoryboard {
        unsafe { NSStoryboard::from_id(msg_send![Self::m_class(), mainStoryboard]) }
    }

    /* Loading the Initial View Controller
     */

    /// Creates the initial view controller or window controller from a storyboard.
    fn im_instantiate_initial_controller(&self) -> id {
        unsafe { msg_send![Self::m_class(), instantiateInitialController] }
    }
}
