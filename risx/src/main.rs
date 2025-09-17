#![no_std]
#![no_main]

use risx::idle_loop;

#[unsafe(no_mangle)]
pub extern "C" fn main(_mhartid: isize, _dtb_ptr: isize) -> ! {
    idle_loop();
}
