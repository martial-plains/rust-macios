use crate::{
    background_tasks::bg_task::IBGTask,
    objective_c_runtime::{
        macros::{interface_impl, object},
        traits::PNSObject,
    },
};

object! {
    /// An object representing a short task typically used to refresh content
    /// that’s run while the app is in the background.
    unsafe pub struct BGAppRefreshTask;
}

#[interface_impl(BGTask)]
impl BGAppRefreshTask {}

impl IBGTask for BGAppRefreshTask {}

impl Default for BGAppRefreshTask {
    fn default() -> Self {
        Self::m_new()
    }
}
