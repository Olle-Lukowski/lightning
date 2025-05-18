#[cfg(target_arch = "riscv32")]
use arbitrary_int::{u4, u22};

#[cfg(target_arch = "riscv64")]
use arbitrary_int::{u4, u36, u44};

use arbitrary_int::{u9, u12, u26};
use bitbybit::{bitenum, bitfield};

use super::instructions::csr;

/// An integer ID of a hardware thread.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MHartId(usize);

impl From<usize> for MHartId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl MHartId {
    const ADDR: u16 = u12::new(0xf14).value();

    /// Get the inner ID.
    pub const fn id(self) -> usize {
        self.0
    }

    /// Read the [`MHartId`] from the CSR.
    ///
    /// # Returns
    ///
    /// The [`MHartId`] value of the current hart.
    ///
    /// # Safety
    ///
    /// The caller must ensure the hart running this code is currently in M-mode.
    ///
    pub unsafe fn read() -> Self {
        // SAFETY: Caller guarantees the safety contract is upheld
        // The `ADDR`` we are using is valid
        unsafe { csr::read::<{ Self::ADDR }>() }.into()
    }
}

/// A field in [`Misa`] that represents what standard extensions are present.
#[bitfield(u26)]
pub struct Extensions {
    /// Atomic extension
    #[bit(0, rw)]
    a: bool,
    /// B extension
    #[bit(1, rw)]
    b: bool,
    /// Compressed extension
    #[bit(2, rw)]
    c: bool,
    /// Double-precision floating-point extension
    #[bit(3, rw)]
    d: bool,
    /// RV32E/64E base ISA
    #[bit(4, rw)]
    e: bool,
    /// Single-precision floating-point extension
    #[bit(5, rw)]
    f: bool,
    /// Reserved
    #[bit(6, rw)]
    g: bool,
    /// Hypervisor extension
    #[bit(7, rw)]
    h: bool,
    /// RV32I/64I base ISA
    #[bit(8, rw)]
    i: bool,
    /// Reserved
    #[bit(9, rw)]
    j: bool,
    /// Reserved
    #[bit(10, rw)]
    k: bool,
    /// Reserved
    #[bit(11, rw)]
    l: bool,
    /// Integer Multiply/Divide extension
    #[bit(12, rw)]
    m: bool,
    /// Tentatively reserved for User-Level Interrupts extension
    #[bit(13, rw)]
    n: bool,
    /// Reserved
    #[bit(14, rw)]
    o: bool,
    /// Tentatively reserved for Packed-SIMD extension
    #[bit(15, rw)]
    p: bool,
    /// Quad-precision floating-point extension
    #[bit(16, rw)]
    q: bool,
    /// Reserved
    #[bit(17, rw)]
    r: bool,
    /// Supervisor mode implemented
    #[bit(18, rw)]
    s: bool,
    /// Reserved
    #[bit(19, rw)]
    t: bool,
    /// User mode implemented
    #[bit(20, rw)]
    u: bool,
    /// Vector extension
    #[bit(21, rw)]
    v: bool,
    /// Reserved
    #[bit(22, rw)]
    w: bool,
    /// Non-standard extensions present
    #[bit(23, rw)]
    x: bool,
    /// Reserved
    #[bit(24, rw)]
    y: bool,
    /// Reserved
    #[bit(25, rw)]
    z: bool,
}

#[bitenum(u2, exhaustive = false)]
pub enum MXLen {
    _32 = 1,
    _64 = 2,
}

#[cfg(target_arch = "riscv32")]
#[bitfield(u32)]
/// A CSR holding information about the currently running ISA.
pub struct Misa {
    /// The standard extensions that are present.
    /// You can disable an extension by setting it's entry in this field to `false`.
    #[bits(0..=25, rw)]
    extensions: Extensions,
    #[bits(26..=29, rw)]
    fill: u4,
    #[bits(30..=31, rw)]
    mxlen: Option<MXLen>,
}

#[cfg(target_arch = "riscv64")]
#[bitfield(u64)]
/// A CSR holding information about the currently running ISA.
pub struct Misa {
    /// The standard extensions that are present.
    /// You can disable an extension by setting it's entry in this field to `false`.
    #[bits(0..=25, rw)]
    extensions: Extensions,
    #[bits(26..=61, rw)]
    fill: u36,
    #[bits(62..=63, rw)]
    mxlen: Option<MXLen>,
}

impl Misa {
    const ADDR: u16 = 0x301;

    /// Read the [`Misa`] from the CSR.
    ///
    /// # Returns
    ///
    /// The [`Misa`] value of the current hart.
    ///
    /// # Safety
    ///
    /// The caller must ensure the hart running this code is currently in M-mode.
    ///
    pub unsafe fn read() -> Self {
        // SAFETY: Caller guarantees the safety contract is upheld
        // The `ADDR`` we are using is valid
        let raw = unsafe { csr::read::<{ Self::ADDR }>() };

        Self::new_with_raw_value(raw as _)
    }

    /// Write the [`Misa`] to the CSR.
    ///
    /// # Safety
    ///
    /// The caller must ensure the hart running this code is currently in M-mode, and
    /// that there are no features enabled that aren't supported,
    /// and no features disabled that are still needed.
    ///
    pub unsafe fn write(self) {
        // SAFETY: Caller guarantees the safety contract is upheld
        // The `ADDR`` we are using is valid
        unsafe { csr::write::<{ Self::ADDR }>(self.raw_value as usize) };
    }
}

#[cfg(target_arch = "riscv32")]
#[bitfield(u32, default = 0)]
/// A CSR holding information about address translation and protection
/// in Supervisor mode.
pub struct Satp {
    /// The physical page number of the root page table.
    #[bits(0..=21, rw)]
    physical_page_number: u22,
    #[bits(22..=30, rw)]
    /// The ID of the address space.
    address_space_id: u9,
    #[bit(31, rw)]
    /// Whether translation and protection is enabled.
    enabled: bool,
}

#[cfg(target_arch = "riscv64")]
#[bitenum(u4, exhaustive = false)]
pub enum TranslationMode {
    Bare = 0,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
}

#[cfg(target_arch = "riscv64")]
#[bitfield(u64, default = 0)]
/// A CSR holding information about address translation and protection
/// in Supervisor mode.
pub struct Satp {
    /// The physical page number of the root page table.
    #[bits(0..=43, rw)]
    physical_page_number: u44,
    #[bits(44..=59, rw)]
    /// The ID of the address space.
    address_space_id: u16,
    #[bits(60..=63, rw)]
    mode: TranslationMode,
}

impl Satp {
    const ADDR: u16 = 0x180;

    /// Read the [`Satp`] from the CSR.
    ///
    /// # Returns
    ///
    /// The [`Satp`] value of the current hart.
    ///
    /// # Safety
    ///
    /// The caller must ensure the hart running this code is currently in M or S-mode.
    ///
    pub unsafe fn read() -> Self {
        // SAFETY: Caller guarantees the safety contract is upheld
        // The `ADDR`` we are using is valid
        let raw = unsafe { csr::read::<{ Self::ADDR }>() };

        Self::new_with_raw_value(raw as _)
    }

    /// Write the [`Satp`] to the CSR.
    ///
    /// # Safety
    ///
    /// The caller must ensure the hart running this code is currently in M or S-mode.
    /// The caller must also ensure that changing the [`Satp`] value
    /// (and thus the current address space) will not violate memory safety.
    ///
    pub unsafe fn write(self) {
        // SAFETY: Caller guarantees the safety contract is upheld
        // The `ADDR`` we are using is valid
        unsafe { csr::write::<{ Self::ADDR }>(self.raw_value as usize) };
    }
}
