#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum NSApplicationDelegateReply {
    Success = 0,
    Cancel = 1,
    Failure = 2,
}
