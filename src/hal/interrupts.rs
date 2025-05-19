use riscv::asm::wfi;

/// Block until the next interrupt.
/// In most cases this will map to a single hardware instruction.
///
pub fn wait() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    wfi()
}
