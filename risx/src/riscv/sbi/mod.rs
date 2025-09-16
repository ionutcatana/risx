// This module includes Rust bindings for OpenSBI calls.
// Each submodule corresponds to an SBI extension.

pub mod base;
pub mod console_debug;
pub mod hart_state_management;
// pub mod ipi;
// pub mod rfence;
// pub mod system_reset;
// pub mod system_suspend;
// pub mod timer;

#[repr(C)]
pub struct SbiRet {
    value: isize,
    error: isize,
}

// An ECALL with an unsupported EID or FID must return NotSupported
#[derive(Debug)]
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

pub enum ResetType {
    Shutdown = 0,
    ColdReboot = 1,
    WarmReboot = 2,
}

pub enum ResetReason {
    NoReason = 0,
    SystemFailure = 1,
}

pub enum SleepType {
    SuspendToRAM = 0,
}
