use core::hint::spin_loop;

use crate::main;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

fn setup() -> ! {
    main();
    loop {
        spin_loop()
    }
}
