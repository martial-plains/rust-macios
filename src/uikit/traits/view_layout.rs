use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSArray,
    objective_c_runtime::traits::{FromId, PNSObject},
    uikit::NSLayoutConstraint,
};

/// The relationship between two user interface objects that must be satisfied by the constraint-based layout system.
pub trait INSLayoutConstraint: PNSObject {
    /* Activating and Deactivating Constraints
     */

    /// The active state of the constraint.
    fn ip_active(&self) -> bool {
        unsafe { msg_send![self.im_self(), isActive] }
    }

    /// Sets the active state of the constraint.
    ///
    /// # Arguments
    ///
    /// * `active` - The active state of the constraint.
    fn ip_set_active(&self, active: bool) {
        unsafe { msg_send![self.im_self(), setActive: active] }
    }

    /// Activates each constraint in the specified array.
    ///
    /// # Arguments
    ///
    /// * `constraints` - The array of constraints to activate.
    fn tm_activate_constraints(constraints: NSArray<NSLayoutConstraint>) {
        unsafe { msg_send![Self::im_class(), activateConstraints: constraints] }
    }

    /// Deactivates each constraint in the specified array.
    ///
    /// # Arguments
    ///
    /// * `constraints` - The array of constraints to deactivate.
    fn tm_deactivate_constraints(constraints: NSArray<NSLayoutConstraint>) {
        unsafe { msg_send![Self::im_class(), deactivateConstraints: constraints] }
    }
}

/// A factory class for creating horizontal layout constraint objects using a fluent API.
pub trait INSLayoutXAxisAnchor: INSLayoutAnchor {
    /* Anchoring to Layout Guides
     */
}

/// A factory class for creating vertical layout constraint objects using a fluent API.
pub trait INSLayoutYAxisAnchor: INSLayoutAnchor {
    /* Anchoring to Layout Guides
     */
}

/// A factory class for creating layout constraint objects using a fluent API.
pub trait INSLayoutAnchor: PNSObject {
    /* Building Constraints
     */

    /// Returns a constraint that defines one item’s attribute as equal to another.
    fn im_constraint_equal_to_anchor<A>(&self, anchor: A) -> NSLayoutConstraint
    where
        A: INSLayoutAnchor,
    {
        unsafe {
            NSLayoutConstraint::from_id(msg_send![self.im_self(), constraintEqualToAnchor: anchor])
        }
    }
}
