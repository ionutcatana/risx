/// Base extension (mandatory for all SBI implementations)
pub mod base;

/// Console debug extension
pub mod console_debug;

/// Hart state management extension
pub mod hart_state_management;

/// Inter-processor interrupt extension
pub mod ipi;

/// Remote fence extension
pub mod rfence;

/// System reset extension
pub mod system_reset;

/// Timer extension
pub mod timer;

#[repr(C)]
/// C struct representing the return value of an SBI call
pub struct SbiRet {
    error: isize,
    value: isize,
}

#[derive(Debug)]
/// SBI error codes
///
/// An ECALL with an unsupported EID or FID must return NotSupported
pub enum ErrorType {
    Failed = -1,
    NotSupported = -2,
    InvalidParam = -3,
    Denied = -4,
    InvalidAddress = -5,
    AlreadyAvailable = -6,
    AlreadyStarted = -7,
    AlreadyStopped = -8,
    NoSharedMemory = -9,
}
