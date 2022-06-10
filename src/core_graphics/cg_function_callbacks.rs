use libc::c_uint;

use super::{CGFunctionEvaluateCallback, CGFunctionReleaseInfoCallback};

/// A structure that contains callbacks needed by a CGFunctionRef object.
#[derive(Debug)]
pub struct CGFunctionCallbacks {
    /// The structure version number. For this structure,the version should be 0.
    pub version: c_uint,
    /// The callback that evaluates the function.
    pub evaluate: Option<*mut CGFunctionEvaluateCallback>,
    /// If non-NULL,the callback used to release the info parameterpassed to CGFunctionCreate.
    pub release_info: Option<*mut CGFunctionReleaseInfoCallback>,
}
