use core::arch::asm;

/// Wait for the next interrupt (WFI instruction).
///
/// # Safety
///
/// This function must only be called when it is safe for the CPU to sleep,
/// and it must be guaranteed that an interrupt will eventually occur to wake it.
/// Must not be called in contexts where sleeping is disallowed (e.g., inside
/// interrupt handlers or while holding locks).
pub unsafe fn wfi() {
    // SAFETY: Caller guarantees the safety contract is upheld
    unsafe { asm!("wfi", options(nomem, nostack, preserves_flags)) }
}

#[cfg(test)]
mod tests {
    use crate::testing::Test;

    use super::*;

    #[test_case]
    const WFI_RETURNS: Test = Test {
        name: "wfi_returns",
        should_fail: false,
        func: wfi_returns,
    };

    fn wfi_returns() -> Result<&'static str, &'static str> {
        unsafe {
            wfi();
        }
        Ok("WFI RETURNED")
    }
}
