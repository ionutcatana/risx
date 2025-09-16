use super::SbiRet;

unsafe extern "C" {
    pub fn sbi_system_reset(reset_type: u32, reset_reason: u32) -> SbiRet;
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
