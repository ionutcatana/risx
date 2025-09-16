use crate::riscv::sbi::ErrorType;

use super::SbiRet;

unsafe extern "C" {
    fn sbi_hart_start(hartid: usize, start_addr: usize, opaque: usize) -> SbiRet;
    fn sbi_hart_stop() -> SbiRet;
    fn sbi_hart_get_status(hartid: usize) -> SbiRet;
    fn sbi_hart_suspend(suspend_type: u32, resume_addr: usize, opaque: usize) -> SbiRet;
}

pub enum HartState {
    Started = 0,
    Stopped = 1,
    StartPending = 2,
    StopPending = 3,
    Suspended = 4,
    SuspendedPending = 5,
    ResumePending = 6,
}

pub enum SuspendType {
    RetentitiveSuspend = 0x00000000,
    NonRetentitiveSuspend = 0x80000000,
}

pub fn start(hartid: usize, start_addr: usize, opaque: usize) -> Result<(), ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_hart_start(hartid, start_addr, opaque);
    }

    match result.error {
        -1 => Err(ErrorType::Failed),
        -3 => Err(ErrorType::InvalidParam),
        -5 => Err(ErrorType::InvalidAddress),
        -6 => Err(ErrorType::AlreadyAvailable),
        _ => Ok(()),
    }
}

pub fn stop() -> Result<(), ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_hart_stop();
    }

    match result.error {
        -1 => Err(ErrorType::Failed),
        _ => Ok(()),
    }
}

pub fn get_status(hartid: usize) -> Result<(), ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_hart_get_status(hartid);
    }

    match result.value {
        -3 => Err(ErrorType::InvalidParam),
        _ => Ok(()),
    }
}

pub fn suspend(
    suspend_type: SuspendType,
    resume_addr: usize,
    opaque: usize,
) -> Result<(), ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_hart_suspend(suspend_type as u32, resume_addr, opaque);
    }

    match result.error {
        -1 => Err(ErrorType::Failed),
        -2 => Err(ErrorType::NotSupported),
        -3 => Err(ErrorType::InvalidParam),
        -5 => Err(ErrorType::InvalidAddress),
        _ => Ok(()),
    }
}
