use super::core::CoreState;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

pub type TrapHandler = fn(Trap);

pub enum Trap {
    // Unknown
    Unknown(usize),

    // Interrupts
    Timer,
    External,
    Software,

    // Exceptions
    Breakpoint,
    IllegalInstruction,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    InstructionFault,
    InstructionMisaligned,
    LoadFault,
    LoadMisaligned,
    StoreFault,
    StoreMisaligned,
    SysCall,
}

fn handle_trap(state: &CoreState, trap: Trap) {
    state.handle_trap(trap)
}

pub fn setup_trap_handler() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv::setup_trap_handler()
}
