use crate::objective_c_runtime::macros::object;

use super::traits::{IBGProcessingTask, IBGTask};

object! {
    /// A time-consuming processing task that runs while the app is in the background.
    unsafe pub struct BGProcessingTask;
}

impl IBGTask for BGProcessingTask {}

impl IBGProcessingTask for BGProcessingTask {}
