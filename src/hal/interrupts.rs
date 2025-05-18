#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use super::arch::riscv::instructions::wfi;

/// Block until the next interrupt.
/// In most cases this will map to a single hardware instruction.
///
/// # Safety
///
#[cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    doc = "See [`wfi`] for details about safety"
)]
pub unsafe fn wait() {
    // SAFETY: Caller guarantees the safety contract is upheld
    unsafe {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        wfi()
    }
}
