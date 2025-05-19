use core::arch::naked_asm;

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
/// # Safety
///
/// See [`MHartId::read`] for details about safety.
///
pub(crate) unsafe fn is_primary_hart() -> bool {
    // SAFETY: Caller guarantees the safety contract is upheld
    mhartid::read() == 0
}
