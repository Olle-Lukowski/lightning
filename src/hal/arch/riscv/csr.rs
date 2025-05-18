use arbitrary_int::u12;

use super::instructions::csr;

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
    /// # Safety
    /// The caller must ensure the hart running this code is currently in M-mode.
    pub unsafe fn read() -> Self {
        // SAFETY: Caller guarantees the safety contract is upheld
        unsafe { csr::read::<{ Self::ADDR }>().into() }
    }
}
