//! Request the system to launch your app in the background to run tasks.

/// Traits for Background Tasks Framework.
pub mod traits;

/* Essentials
 */

mod bg_app_refresh_task;
mod bg_app_refresh_task_request;
mod bg_processing_task;
mod bg_processing_task_request;
mod bg_task_scheduler;

pub use bg_app_refresh_task::*;
pub use bg_app_refresh_task_request::*;
pub use bg_processing_task::*;
pub use bg_processing_task_request::*;
pub use bg_task_scheduler::*;
