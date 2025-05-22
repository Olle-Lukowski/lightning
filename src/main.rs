#![no_std]
#![no_main]

use core::{hint::spin_loop, panic::PanicInfo};

use hal::{interrupts, trap::Trap};

mod hal;

pub fn main() -> ! {
    loop {
        interrupts::wait()
    }
}

pub fn handle_trap(_trap: Trap) {}

#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {
        spin_loop()
    }
}
