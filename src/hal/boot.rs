use crate::main;

use super::interrupts;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

fn setup() -> ! {
    main();
    loop {
        // SAFETY: We are done setting everything up, we are not holding any locks.
        // It is safe to block. Even if interrupts are disabled,
        // we are fine with not continuing past this point.
        unsafe { interrupts::wait() }
    }
}
