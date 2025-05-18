#![no_std]
#![no_main]

use core::{hint::spin_loop, panic::PanicInfo};

mod hal;

pub fn main() {}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {
        spin_loop()
    }
}
