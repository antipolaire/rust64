#![no_std]
#![feature(start)]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use ufmt_stdio::*;
use mos_hardware::*;

#[start]
fn _main(_argc: isize, _argv: *const *const u8) -> isize {
    print!("HELLO FROM RUST!");
    0
}
 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(not(target_vendor = "nes-nrom-128"))]
    print!("!");
    loop {}
}