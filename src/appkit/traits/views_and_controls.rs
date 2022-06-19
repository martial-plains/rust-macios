use objc::{msg_send, runtime::Sel, sel, sel_impl};

use crate::{
    appkit::{NSImage, NSMenuItem, NSView, NSWindow},
    foundation::{NSArray, NSCoder, NSRect, NSString},
    objective_c_runtime::{id, traits::FromId},
    uikit::{NSLayoutXAxisAnchor, NSLayoutYAxisAnchor},
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

    /* Configuring the view
     */

    /// The view that is the parent of the current view.
    fn ip_superview(&self) -> NSView {
        unsafe { NSView::from_id(msg_send![self.im_self(), superview]) }
    }

    /// The array of views embedded in the current view.
    fn ip_subviews<V>(&self) -> NSArray<V>
    where
        V: INSView,
    {
        unsafe { NSArray::from_id(msg_send![self.im_self(), subviews]) }
    }

    /// The view’s window object, if it is installed in a window.
    fn ip_window(&self) -> NSWindow {
        unsafe { NSWindow::from_id(msg_send![self.im_self(), window]) }
    }

    /// The view’s closest opaque ancestor, which might be the view itself.
    fn ip_opaque_ancestor(&self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.im_self(), opaqueAncestor]) }
    }

    /// Returns a Boolean value that indicates whether the view is a subview of the specified view.
    fn ip_is_descendant_of(&self, view: NSView) -> bool {
        unsafe { msg_send![self.im_self(), isDescendantOfView: view] }
    }

    /// Returns the closest ancestor shared by the view and another specified view.
    fn ip_ancestor_shared_with_view(&self, view: NSView) -> NSView {
        unsafe { NSView::from_id(msg_send![self.im_self(), ancestorSharedWithView: view]) }
    }

    /// The menu item containing the view or any of its superviews in the view hierarchy.
    fn ip_enclosing_menu_item(&self) -> NSMenuItem {
        unsafe { NSMenuItem::from_id(msg_send![self.im_self(), enclosingMenuItem]) }
    }

    /* Adding and Removing Subviews
     */

    /// Adds a view to the view’s subviews so it’s displayed above its siblings.
    fn ip_add_subview<V>(&self, view: V)
    where
        V: INSView,
    {
        unsafe { msg_send![self.im_self(), addSubview: view] }
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

    /* Opting In to Auto Layout
     */

    /// Returns a Boolean value indicating whether the view depends on the constraint-based layout system.
    fn tp_requires_constraint_based_layout() -> bool {
        unsafe { msg_send![Self::im_class(), requiresConstraintBasedLayout] }
    }

    /// A Boolean value indicating whether the view’s autoresizing mask is translated into constraints for the constraint-based layout system.
    fn ip_translates_autoresizing_mask_to_constraints(&self) -> bool {
        unsafe { msg_send![self.im_self(), translatesAutoresizingMaskIntoConstraints] }
    }

    /// Sets a Boolean value indicating whether the view’s autoresizing mask is translated into constraints for the constraint-based layout system.
    ///
    /// # Arguments
    ///
    /// * `flag` - The new value.
    fn ip_set_translates_autoresizing_mask_to_constraints(&self, flag: bool) {
        unsafe {
            msg_send![
                self.im_self(),
                setTranslatesAutoresizingMaskIntoConstraints: flag
            ]
        }
    }

    /* Creating Constraints Using Layout Anchors
     */

    /// A layout anchor representing the bottom edge of the view’s frame.
    fn ip_bottom_anchor(&self) -> NSLayoutYAxisAnchor {
        unsafe { NSLayoutYAxisAnchor::from_id(msg_send![self.im_self(), bottomAnchor]) }
    }

    /// A layout anchor representing the horizontal center of the view’s frame.
    fn ip_center_x_anchor(&self) -> NSLayoutXAxisAnchor {
        unsafe { NSLayoutXAxisAnchor::from_id(msg_send![self.im_self(), centerXAnchor]) }
    }

    /// A layout anchor representing the vertical center of the view’s frame.
    fn ip_center_y_anchor(&self) -> NSLayoutYAxisAnchor {
        unsafe { NSLayoutYAxisAnchor::from_id(msg_send![self.im_self(), centerYAnchor]) }
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
    fn ip_title(&self) -> NSString {
        unsafe { msg_send![self.im_self(), title] }
    }

    /// Sets the title displayed on the button when it’s in an off state.
    fn ip_set_title(&self, title: NSString) {
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
