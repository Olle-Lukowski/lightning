use core::arch::naked_asm;

use super::setup;

#[unsafe(no_mangle)]
#[unsafe(naked)]
extern "C" fn _start() -> ! {
    #[allow(unused_unsafe)]
    unsafe {
        naked_asm!(
            ".option push",
            ".option norelax",
            "la gp, __global_pointer$",
            ".option pop",
            "la sp, __stack_start",
            "j {}",
            sym setup,
        )
    };
}
