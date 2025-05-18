use crate::main;

use super::interrupts;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

/// Check whether the currently running core is the primary one of the system.
///
/// # Safety
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
        // TODO
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
