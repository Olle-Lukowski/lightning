#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub mod riscv;

/// The execution environment used by the kernel.
pub trait Environment {
    /// Activate the execution environment, combined with a call to the kernel entry point
    /// ([`crate::main`]).
    ///
    /// # Safety
    ///
    /// Different implementors may have different safety requirements, check their docs.
    ///
    unsafe fn activate(&self);
}
