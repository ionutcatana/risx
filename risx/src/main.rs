#![no_std]
#![no_main]

use risx as _;
use risx::idle_loop;
use risx::riscv::sbi::util::extension_check;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    extension_check();
    idle_loop();
}
