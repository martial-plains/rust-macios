use crate::objective_c_runtime::traits::t_NSObject;

/// A container for information broadcast through a notification center to all registered observers.
pub trait t_NSNotification: t_NSObject {
    /// Initializes an empty notification.
    fn init() -> Self;
}
