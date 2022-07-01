use crate::{background_tasks::traits::IBGTask, objective_c_runtime::macros::object};

object! {
    /// An object representing a short task typically used to refresh content
    /// that’s run while the app is in the background.
    unsafe pub struct BGAppRefreshTask;
}

impl IBGTask for BGAppRefreshTask {}
