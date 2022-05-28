use crate::objective_c_runtime::traits::PNSObject;

/// A container for information broadcast through a notification center to all registered observers.
pub trait t_NSNotification: PNSObject {
    /// Initializes an empty notification.
    fn init() -> Self;
}
