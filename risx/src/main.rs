#![no_std]
#![no_main]

use risx as _;
use risx::idle_loop;
use risx::riscv::sbi::debug_console::write;

#[unsafe(no_mangle)]
pub extern "C" fn main(_hartid: usize, _dtb: usize) -> ! {
    write("Hello, world!\n").unwrap();
    idle_loop();
}
