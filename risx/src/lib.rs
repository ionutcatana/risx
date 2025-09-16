#![no_std]

use crate::riscv::sbi::console_debug::write;

/// This module includes RISC-V specific functionality.
pub mod riscv;

#[panic_handler]
/// Print a panic message and halt the CPU.
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    let _ = write("panic!\n");
    loop {}
}

/// An idle loop that does nothing and never returns.
pub fn idle_loop() -> ! {
    loop {}
}
