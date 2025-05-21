use riscv::register::mhartid;

/// Checks if the current hart is the primary one.
///
/// # Returns
///
/// Whether the hart running this code is the primary hart.
///
pub(crate) fn is_primary_hart() -> bool {
    mhartid::read() == 0
}
