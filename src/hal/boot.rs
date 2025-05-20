use core::slice;

use crate::{
    hal::execution::{self, Environment as _},
    main,
};

use super::interrupts;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

/// Check whether the currently running core is the primary one of the system.
pub fn is_primary_core() -> bool {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv::is_primary_hart()
}

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

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    // SAFETY: TODO
    unsafe {
        execution::riscv::ExecutionEnvironment::new().activate()
    };

    main();

    park()
}

fn park() -> ! {
    loop {
        interrupts::wait()
    }
}
