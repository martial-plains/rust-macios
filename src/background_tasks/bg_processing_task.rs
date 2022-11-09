use crate::{
    background_tasks::bg_task::IBGTask,
    object,
    objective_c_runtime::{macros::interface_impl, traits::PNSObject},
};

object! {
    /// A time-consuming processing task that runs while the app is in the background.
    unsafe pub struct BGProcessingTask;
}

#[interface_impl(BGTask)]
impl BGProcessingTask {}

impl IBGTask for BGProcessingTask {}

impl Default for BGProcessingTask {
    fn default() -> Self {
        Self::m_new()
    }
}
