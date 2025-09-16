use super::{ErrorType, SbiRet};

unsafe extern "C" {
    fn sbi_get_spec_version() -> SbiRet;
    fn sbi_get_impl_id() -> SbiRet;
    fn sbi_get_impl_version() -> SbiRet;
    fn sbi_probe_extension(extension_id: isize) -> SbiRet;
    fn sbi_get_mvendorid() -> SbiRet;
    fn sbi_get_marchid() -> SbiRet;
    fn sbi_get_mimpid() -> SbiRet;
}

#[repr(C)]
pub enum ImplementationId {
    /* 0 */ BerkeleyBootLoader,
    /* 1 */ OpenSBI,
    /* 2 */ Xvisor,
    /* 3 */ KVM,
    /* 4 */ RustSBI,
    /* 5 */ Diosix,
    /* 6 */ Coffer,
    /* 7 */ XerProject,
    /* 8 */ PolarFireHartSoftwareServices,
}

pub enum Extension {
    Base = 0x10,
    Timer = 0x54494D45,
    InterProcessorInterrupt = 0x735049,
    RemoteFence = 0x52464E43,
    HartStateManagement = 0x48534D,
    SystemReset = 0x53525354,
    ConsoleDebug = 0x4442434E,
    SystemSuspend = 0x53555350,
    CollaborativeProcessorPerformanceControl = 0x43505043,
    NestedAcceleration = 0x4E41434C,
    StealTimeAccounting = 0x535441,
}

pub fn get_spec_version() -> Result<(isize, isize), ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_get_spec_version();
    }

    let minor_version = result.value & 0x00FFFFFF;
    let major_version = result.value >> 24;

    Ok((major_version, minor_version))
}

pub fn get_impl_id() -> Result<isize, ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_get_impl_id();
    }

    Ok(result.value)
}

pub fn get_impl_version() -> Result<isize, ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_get_impl_version();
    }

    Ok(result.value)
}

pub fn probe_extension(extension: Extension) -> Result<bool, ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_probe_extension(extension as isize);
    }

    Ok(!result.value == 0)
}

pub fn get_mvendorid() -> Result<isize, ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_get_mvendorid();
    }

    let mvendorid = result.value;
    Ok(mvendorid)
}

pub fn get_marchid() -> Result<isize, ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_get_marchid();
    }

    let marchid = result.value;
    Ok(marchid)
}

pub fn get_mimpid() -> Result<Option<ImplementationId>, ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_get_mimpid();
    }

    match result.value {
        0 => Ok(Some(ImplementationId::BerkeleyBootLoader)),
        1 => Ok(Some(ImplementationId::OpenSBI)),
        2 => Ok(Some(ImplementationId::Xvisor)),
        3 => Ok(Some(ImplementationId::KVM)),
        4 => Ok(Some(ImplementationId::RustSBI)),
        5 => Ok(Some(ImplementationId::Diosix)),
        6 => Ok(Some(ImplementationId::Coffer)),
        7 => Ok(Some(ImplementationId::XerProject)),
        8 => Ok(Some(ImplementationId::PolarFireHartSoftwareServices)),
        _ => Ok(None),
    }
}
