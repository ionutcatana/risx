use super::{ErrorType, SbiRet};

unsafe extern "C" {
    fn sbi_debug_console_write(
        num_bytes: usize,
        base_addr_lo: usize,
        base_addr_hi: usize,
    ) -> SbiRet;
    fn sbi_debug_console_read(num_bytes: usize, base_addr_lo: usize, base_addr_hi: usize)
    -> SbiRet;
    fn sbi_debug_console_write_byte(byte: u8) -> SbiRet;
}

pub fn write(message: &str) -> Result<(), ErrorType> {
    let result: SbiRet;

    unsafe {
        result = sbi_debug_console_write(message.len(), message.as_ptr() as usize, 0);
    }

    match result.error {
        0 => Ok(()),
        -1 => Err(ErrorType::Failed),
        -3 => Err(ErrorType::InvalidParam),
        -4 => Err(ErrorType::Denied),
        _ => panic!(),
    }
}

pub fn read(buffer: &mut [u8], num_bytes: usize) -> Result<(), ErrorType> {
    let result: SbiRet;

    unsafe {
        result = sbi_debug_console_read(num_bytes, buffer.as_ptr() as usize, 0);
    }

    match result.error {
        0 => Ok(()),
        -1 => Err(ErrorType::Failed),
        -3 => Err(ErrorType::InvalidParam),
        -4 => Err(ErrorType::Denied),
        _ => panic!(),
    }
}

pub fn write_byte(byte: u8) -> Result<(), ErrorType> {
    let result: SbiRet;
    unsafe {
        result = sbi_debug_console_write_byte(byte);
    }

    match result.error {
        0 => Ok(()),
        -1 => Err(ErrorType::Failed),
        -4 => Err(ErrorType::Denied),
        _ => panic!(),
    }
}
