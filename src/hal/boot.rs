use core::slice;

use crate::hal::{
    core::{is_primary_core, load_boot_core_state},
    execution::{self, Environment as _},
    trap::setup_trap_handler,
};

use super::interrupts;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

fn setup() -> ! {
    if !is_primary_core() {
        park()
    }

    // SAFETY: We depend on the symbols being properly defined at link time.
    unsafe extern "C" {
        unsafe static mut __bss_start: u8;
        unsafe static __bss_end: u8;
    }

    // SAFETY: We depend on the symbols being properly defined at link time.
    let bss = unsafe {
        slice::from_raw_parts_mut(
            &raw mut __bss_start,
            (&raw const __bss_end)
                .offset_from(&raw const __bss_start)
                .try_into()
                .expect("bss start is before bss end!"),
        )
    };

    bss.fill(0);

    load_boot_core_state();

    setup_trap_handler();

    // Everything is set up, kernel time! (activating the environment will jump to the kernel)
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    // SAFETY: Trap handler is properly set up.
    unsafe {
        execution::riscv::ExecutionEnvironment::new().activate()
    };

    park()
}

fn park() -> ! {
    loop {
        interrupts::wait()
    }
}
