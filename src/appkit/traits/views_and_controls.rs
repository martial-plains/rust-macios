use objc::{msg_send, runtime::Sel, sel, sel_impl};

use crate::{
    appkit::NSImage,
    foundation::{NSCoder, NSRect, NSString},
    objective_c_runtime::{id, traits::FromId},
};

use super::INSResponder;

/// The infrastructure for drawing, printing, and handling events in an app.
pub trait INSView: INSResponder {
    /// Initializes and returns a newly allocated NSView object with a specified frame rectangle.
    fn im_init_with_frame(frame: NSRect) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let obj: id = msg_send![Self::im_class(), alloc];
            Self::from_id(msg_send![obj, initWithFrame: frame])
        }
    }

    /// Initializes a view using from data in the specified coder object.
    fn im_init_with_coder(coder: NSCoder) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let obj: id = msg_send![Self::im_class(), alloc];
            Self::from_id(msg_send![obj, initWithCoder: coder])
        }
    }

    /// Restores the view to an initial state so that it can be reused.
    fn im_prepare_for_reuse(&self) {
        unsafe { msg_send![self.im_self(), prepareForReuse] }
    }

    /* Modifying the Bounds Rectangle
     */

    /// The view’s bounds rectangle, which expresses its location and size in its own coordinate system.
    fn ip_bounds(&self) -> NSRect {
        unsafe { msg_send![self.im_self(), bounds] }
    }

    /// Sets the view’s bounds rectangle, which expresses its location and size in its own coordinate system.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The new bounds rectangle.
    fn ip_set_bounds(&self, bounds: NSRect) {
        unsafe { msg_send![self.im_self(), setBounds: bounds] }
    }
}

/// A specialized view, such as a button or text field, that notifies your app of relevant events using the target-action design pattern.
pub trait INSControl: INSView {
    /*  Implementing the Target-Action Mechanism */

    /// The default action-message selector associated with the control.
    fn ip_action(&self) -> Sel {
        unsafe { msg_send![self.im_self(), action] }
    }

    /// Sets the default action-message selector associated with the control.
    ///
    /// # Arguments
    ///
    /// * `action` - The new action-message selector.
    fn ip_set_action(&self, action: Sel) {
        unsafe { msg_send![self.im_self(), setAction: action] }
    }
}

/// A control that defines an area on the screen that a user clicks to trigger an action.
pub trait INSButton: INSControl {
    /* Configuring Buttons
     */

    /// The title displayed on the button when it’s in an off state.
    fn title(&self) -> NSString {
        unsafe { msg_send![self.im_self(), title] }
    }

    /// Sets the title displayed on the button when it’s in an off state.
    fn set_title(&self, title: NSString) {
        unsafe { msg_send![self.im_self(), setTitle: title] }
    }

    /* Configuring Button Images
     */

    /// The image that appears on the button when it’s in an off state, or nil if there is no such image.
    fn ip_image(&self) -> NSImage {
        unsafe { NSImage::from_id(msg_send![self.im_self(), image]) }
    }

    /// Sets the image that appears on the button when it’s in an off state, or nil if there is no such image.
    ///
    /// # Arguments
    ///
    /// * `image` - The new image.
    fn ip_set_image(&self, image: NSImage) {
        unsafe { msg_send![self.im_self(), setImage: image] }
    }
}
