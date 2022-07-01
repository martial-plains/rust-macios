use crate::objective_c_runtime::macros::object;

use super::traits::IBGTaskScheduler;

object! {
    /// A class for scheduling task requests that launch your app in the background.
    unsafe pub struct BGTaskScheduler;
}

impl IBGTaskScheduler for BGTaskScheduler {}
