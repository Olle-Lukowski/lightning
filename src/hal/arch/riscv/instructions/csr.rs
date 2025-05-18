use core::arch::asm;

/// Read from a Control and Status Register (CSR) at the given address.
///
/// # Safety
///
/// - The caller must ensure that `ADDR` is a valid CSR address and that reading
///   from it is safe in the current context.
/// - Reading certain CSRs may have side effects or may not be allowed in all privilege levels.
///
/// # Panics
///
/// Panics in debug mode if `ADDR` does not fit in the 12-bit immediate field
/// of the RISC-V instruction encoding (i.e., if `ADDR >= 1 << 12`).
pub unsafe fn read<const ADDR: u16>() -> usize {
    debug_assert!(ADDR < (1 << 12), "CSR address must be 12 bits");
    let result: usize;
    // SAFETY: Caller ensures `ADDR`` is valid and access is allowed
    unsafe { asm!("csrr {result}, {addr}", result = lateout(reg) result, addr = const ADDR) };
    result
}

/// Write to a Control and Status Register (CSR) at the given address.
///
/// # Returns
///
/// The current value of the CSR.
///
/// # Safety
///
/// - The caller must ensure that `ADDR` is a valid CSR address and that writing
///   to it is safe in the current context.
/// - The caller must ensure that the CSR at `ADDR` is writable.
/// - Writing to certain CSRs may affect system behavior or may not be allowed in all privilege levels.
///
/// # Panics
///
/// Panics in debug mode if `ADDR` does not fit in the 12-bit immediate field
/// of the RISC-V instruction encoding (i.e., if `ADDR >= 1 << 12`).
pub unsafe fn write<const ADDR: u16>(value: usize) {
    debug_assert!(ADDR < (1 << 12), "CSR address must be 12 bits");
    unsafe { asm!("csrw {addr}, {value}", addr = const ADDR, value = in(reg) value) }
}

/// Atomically sets bits in a CSR using logical OR.
///
/// # Safety
///
///
/// - The caller must ensure that `ADDR` is a valid CSR address and that writing
///   to it is safe in the current context.
/// - The caller must ensure that the CSR at `ADDR` is writable.
/// - Writing to certain CSRs may affect system behavior or may not be allowed in all privilege levels.
///
/// # Panics
///
/// Panics in debug mode if `ADDR` does not fit in the 12-bit immediate field
/// of the RISC-V instruction encoding (i.e., if `ADDR >= 1 << 12`).
pub unsafe fn set<const ADDR: u16>(mask: usize) {
    debug_assert!(ADDR < (1 << 12), "CSR address must be 12 bits");
    unsafe {
        asm!(
            "csrs {addr}, {mask}",
            addr = const ADDR,
            mask = in(reg) mask,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Atomically clears bits in a CSR using logical AND with inverted mask.
///
/// # Safety
///
///
/// - The caller must ensure that `ADDR` is a valid CSR address and that writing
///   to it is safe in the current context.
/// - The caller must ensure that the CSR at `ADDR` is writable.
/// - Writing to certain CSRs may affect system behavior or may not be allowed in all privilege levels.
///
/// # Panics
///
/// Panics in debug mode if `ADDR` does not fit in the 12-bit immediate field
/// of the RISC-V instruction encoding (i.e., if `ADDR >= 1 << 12`).
pub unsafe fn clear<const ADDR: u16>(mask: usize) {
    debug_assert!(ADDR < (1 << 12), "CSR address must be 12 bits");
    unsafe {
        asm!(
            "csrc {addr}, {mask}",
            addr = const ADDR,
            mask = in(reg) mask,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// Atomically reads a CSR and sets bits using logical OR.
///
/// # Returns
///
/// The original value of the CSR before modification.
///
/// # Safety
///
///
/// - The caller must ensure that `ADDR` is a valid CSR address and that writing
///   to it is safe in the current context.
/// - The caller must ensure that the CSR at `ADDR` is writable.
/// - Writing to certain CSRs may affect system behavior or may not be allowed in all privilege levels.
///
/// # Panics
///
/// Panics in debug mode if `ADDR` does not fit in the 12-bit immediate field
/// of the RISC-V instruction encoding (i.e., if `ADDR >= 1 << 12`).
pub unsafe fn read_set<const ADDR: u16>(mask: usize) -> usize {
    debug_assert!(ADDR < (1 << 12), "CSR address must be 12 bits");
    let result: usize;
    unsafe {
        asm!(
            "csrrs {result}, {addr}, {mask}",
            result = lateout(reg) result,
            addr = const ADDR,
            mask = in(reg) mask,
            options(nomem, nostack, preserves_flags)
        );
    }
    result
}

/// Atomically reads a CSR and clears bits using logical AND with inverted mask.
///
/// # Returns
///
/// The original value of the CSR before modification.
///
/// # Safety
///
///
/// - The caller must ensure that `ADDR` is a valid CSR address and that writing
///   to it is safe in the current context.
/// - The caller must ensure that the CSR at `ADDR` is writable.
/// - Writing to certain CSRs may affect system behavior or may not be allowed in all privilege levels.
///
/// # Panics
///
/// Panics in debug mode if `ADDR` does not fit in the 12-bit immediate field
/// of the RISC-V instruction encoding (i.e., if `ADDR >= 1 << 12`).
pub unsafe fn read_clear<const ADDR: u16>(mask: usize) -> usize {
    debug_assert!(ADDR < (1 << 12), "CSR address must be 12 bits");
    let result: usize;
    unsafe {
        asm!(
            "csrrc {result}, {addr}, {mask}",
            result = lateout(reg) result,
            addr = const ADDR,
            mask = in(reg) mask,
            options(nomem, nostack, preserves_flags)
        );
    }
    result
}
