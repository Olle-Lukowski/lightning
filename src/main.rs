#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(testing::runner)]

use core::{hint::spin_loop, panic::PanicInfo};

mod hal;
mod testing;

pub fn main() {
    #[cfg(test)]
    test_main();
}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {
        spin_loop()
    }
}
