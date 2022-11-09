use objc::{msg_send, sel, sel_impl};

use crate::{
    foundation::NSArray,
    object,
    objective_c_runtime::{macros::interface_impl, traits::PNSObject},
};

object! {
    /// The relationship between two user interface objects that must be satisfied by the constraint-based layout system.
    unsafe pub struct NSLayoutConstraint;
}

#[interface_impl(NSObject)]
impl NSLayoutConstraint {
    /* Activating and Deactivating Constraints
     */

    /// The active state of the constraint.
    #[property]
    pub fn active(&self) -> bool {
        unsafe { msg_send![self.m_self(), isActive] }
    }

    /// Sets the active state of the constraint.
    ///
    /// # Arguments
    ///
    /// * `active` - The active state of the constraint.
    #[property]
    pub fn set_active(&self, active: bool) {
        unsafe { msg_send![self.m_self(), setActive: active] }
    }

    /// Activates each constraint in the specified array.
    ///
    /// # Arguments
    ///
    /// * `constraints` - The array of constraints to activate.
    #[method]
    pub fn activate_constraints(constraints: NSArray<NSLayoutConstraint>) {
        unsafe { msg_send![Self::m_class(), activateConstraints: constraints] }
    }

    /// Deactivates each constraint in the specified array.
    ///
    /// # Arguments
    ///
    /// * `constraints` - The array of constraints to deactivate.
    #[method]
    pub fn deactivate_constraints(constraints: NSArray<NSLayoutConstraint>) {
        unsafe { msg_send![Self::m_class(), deactivateConstraints: constraints] }
    }
}
