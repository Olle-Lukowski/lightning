use core::arch::naked_asm;

#[cfg(feature = "riscv_zicsr")]
use riscv::register::mhartid;

use super::setup;

#[unsafe(no_mangle)]
#[unsafe(naked)]
#[unsafe(link_section = ".text.init")]
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

/// Checks if the current hart is the primary one.
///
/// # Returns
///
/// Whether the hart running this code is the primary hart.
///
pub(crate) fn is_primary_hart() -> bool {
    #[cfg(feature = "riscv_zicsr")]
    return mhartid::read() == 0;
    #[cfg(not(feature = "riscv_zicsr"))]
    return true; // We can't check if this is the primary hart
}
