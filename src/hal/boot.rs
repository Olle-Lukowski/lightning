use core::slice;

use crate::main;

use super::interrupts;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

/// Check whether the currently running core is the primary one of the system.
///
/// # Safety
///
#[cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    doc = "See [`riscv::is_primary_hart`] for details about safety."
)]
pub unsafe fn is_primary_core() -> bool {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        riscv::is_primary_hart()
    }
}

fn setup() -> ! {
    main();

    // SAFETY:
    // RISC-V:
    //   We are running in M-mode.
    if unsafe { is_primary_core() } {
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
    }

    loop {
        // SAFETY:
        // RISC-V:
        //   We are done setting everything up, we are not holding any locks.
        //   It is safe to block. Even if interrupts are disabled,
        //   we are fine with not continuing past this point.
        unsafe { interrupts::wait() }
    }
}
