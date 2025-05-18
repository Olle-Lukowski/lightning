use core::arch::asm;

pub mod csr;

/// Wait for the next interrupt (WFI instruction).
///
/// # Safety
///
/// This function must only be called when it is safe for the CPU to sleep,
/// and it must be guaranteed that an interrupt will eventually occur to wake it.
/// Must not be called in contexts where sleeping is disallowed (e.g., inside
/// interrupt handlers or while holding locks).
///
pub unsafe fn wfi() {
    // SAFETY: Caller guarantees the safety contract is upheld
    unsafe { asm!("wfi", options(nomem, nostack, preserves_flags)) }
}
