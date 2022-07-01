use crate::objective_c_runtime::macros::object;

use super::traits::{IBGProcessingTaskRequest, IBGTaskRequest};

object! {
    /// A request to launch your app in the background to execute a processing task that can take minutes to complete.
    unsafe pub struct BGProcessingTaskRequest;
}

impl IBGTaskRequest for BGProcessingTaskRequest {}

impl IBGProcessingTaskRequest for BGProcessingTaskRequest {}
