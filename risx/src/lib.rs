#![no_std]

pub mod riscv;

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn idle_loop() -> ! {
    loop {}
}
