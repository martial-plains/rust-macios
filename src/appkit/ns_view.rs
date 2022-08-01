use objc::{msg_send, sel, sel_impl};
use objective_c_runtime_proc_macros::interface_impl;

use crate::{
    foundation::{NSArray, NSCoder, NSRect},
    objective_c_runtime::{id, traits::FromId},
    uikit::{NSLayoutXAxisAnchor, NSLayoutYAxisAnchor},
};

use super::{object, INSResponder, NSMenuItem, NSWindow};

object! {
    /// The infrastructure for drawing, printing, and handling events in an app.
    unsafe pub struct NSView;
}

impl INSResponder for NSView {}

#[interface_impl(NSResponder)]
impl NSView {
    /// Initializes and returns a newly allocated NSView object with a specified frame rectangle.
    #[method]
    pub fn init_with_frame(frame: NSRect) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let obj: id = msg_send![Self::m_class(), alloc];
            Self::from_id(msg_send![obj, initWithFrame: frame])
        }
    }

    /// Initializes a view using from data in the specified coder object.
    #[method]
    pub fn init_with_coder(coder: NSCoder) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe {
            let obj: id = msg_send![Self::m_class(), alloc];
            Self::from_id(msg_send![obj, initWithCoder: coder])
        }
    }

    /// Restores the view to an initial state so that it can be reused.
    #[method]
    pub fn prepare_for_reuse(&self) {
        unsafe { msg_send![self.m_self(), prepareForReuse] }
    }

    /* Configuring the view
     */

    /// The view that is the parent of the current view.
    #[property]
    pub fn superview(&self) -> NSView {
        unsafe { NSView::from_id(msg_send![self.m_self(), superview]) }
    }

    /// The array of views embedded in the current view.
    #[property]
    pub fn subviews<V>(&self) -> NSArray<V>
    where
        V: INSView,
    {
        unsafe { NSArray::from_id(msg_send![self.m_self(), subviews]) }
    }

    /// The view’s window object, if it is installed in a window.
    #[property]
    pub fn window(&self) -> NSWindow {
        unsafe { NSWindow::from_id(msg_send![self.m_self(), window]) }
    }

    /// The view’s closest opaque ancestor, which might be the view itself.
    #[property]
    pub fn opaque_ancestor(&self) -> Self
    where
        Self: Sized + FromId,
    {
        unsafe { Self::from_id(msg_send![self.m_self(), opaqueAncestor]) }
    }

    /// Returns a Boolean value that indicates whether the view is a subview of the specified view.
    #[property]
    pub fn is_descendant_of(&self, view: NSView) -> bool {
        unsafe { msg_send![self.m_self(), isDescendantOfView: view] }
    }

    /// Returns the closest ancestor shared by the view and another specified view.
    #[property]
    pub fn ancestor_shared_with_view(&self, view: NSView) -> NSView {
        unsafe { NSView::from_id(msg_send![self.m_self(), ancestorSharedWithView: view]) }
    }

    /// The menu item containing the view or any of its superviews in the view hierarchy.
    #[property]
    pub fn enclosing_menu_item(&self) -> NSMenuItem {
        unsafe { NSMenuItem::from_id(msg_send![self.m_self(), enclosingMenuItem]) }
    }

    /* Adding and Removing Subviews
     */

    /// Adds a view to the view’s subviews so it’s displayed above its siblings.
    #[property]
    pub fn add_subview<V>(&self, view: V)
    where
        V: INSView,
    {
        unsafe { msg_send![self.m_self(), addSubview: view] }
    }

    /* Modifying the Bounds Rectangle
     */

    /// The view’s bounds rectangle, which expresses its location and size in its own coordinate system.
    #[property]
    pub fn bounds(&self) -> NSRect {
        unsafe { msg_send![self.m_self(), bounds] }
    }

    /// Sets the view’s bounds rectangle, which expresses its location and size in its own coordinate system.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The new bounds rectangle.
    #[property]
    pub fn set_bounds(&self, bounds: NSRect) {
        unsafe { msg_send![self.m_self(), setBounds: bounds] }
    }

    /* Opting In to Auto Layout
     */

    /// Returns a Boolean value indicating whether the view depends on the constraint-based layout system.
    #[property]
    pub fn requires_constraint_based_layout() -> bool {
        unsafe { msg_send![Self::m_class(), requiresConstraintBasedLayout] }
    }

    /// A Boolean value indicating whether the view’s autoresizing mask is translated into constraints for the constraint-based layout system.
    #[property]
    pub fn translates_autoresizing_mask_to_constraints(&self) -> bool {
        unsafe { msg_send![self.m_self(), translatesAutoresizingMaskIntoConstraints] }
    }

    /// Sets a Boolean value indicating whether the view’s autoresizing mask is translated into constraints for the constraint-based layout system.
    ///
    /// # Arguments
    ///
    /// * `flag` - The new value.
    #[property]
    pub fn set_translates_autoresizing_mask_to_constraints(&self, flag: bool) {
        unsafe {
            msg_send![
                self.m_self(),
                setTranslatesAutoresizingMaskIntoConstraints: flag
            ]
        }
    }

    /* Creating Constraints Using Layout Anchors
     */

    /// A layout anchor representing the bottom edge of the view’s frame.
    #[property]
    pub fn bottom_anchor(&self) -> NSLayoutYAxisAnchor {
        unsafe { NSLayoutYAxisAnchor::from_id(msg_send![self.m_self(), bottomAnchor]) }
    }

    /// A layout anchor representing the horizontal center of the view’s frame.
    #[property]
    pub fn center_x_anchor(&self) -> NSLayoutXAxisAnchor {
        unsafe { NSLayoutXAxisAnchor::from_id(msg_send![self.m_self(), centerXAnchor]) }
    }

    /// A layout anchor representing the vertical center of the view’s frame.
    #[property]
    pub fn center_y_anchor(&self) -> NSLayoutYAxisAnchor {
        unsafe { NSLayoutYAxisAnchor::from_id(msg_send![self.m_self(), centerYAnchor]) }
    }
}
