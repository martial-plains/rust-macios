/// Constants that indicate whether a copy or print operation was successful,
/// was canceled, or failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSApplicationDelegateReply {
    /// Indicates the operation succeeded.
    Success = 0,
    /// Indicates the user cancelled the operation.
    Cancel = 1,
    /// Indicates an error occurred processing the operation.
    Failure = 2,
}
