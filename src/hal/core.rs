use crate::handle_trap;

use super::trap::{Trap, TrapHandler};

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

/// Check whether the currently running core is the primary one of the system.
pub fn is_primary_core() -> bool {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv::is_primary_hart()
}

pub struct CoreState {
    pub trap_handler: TrapHandler,
}

impl CoreState {
    pub fn handle_trap(&self, trap: Trap) {
        (self.trap_handler)(trap)
    }
}

static BOOT_CORE_STATE: CoreState = CoreState {
    trap_handler: handle_trap,
};

pub fn load_boot_core_state() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        use ::riscv::register::mscratch;
        mscratch::write((&BOOT_CORE_STATE) as *const _ as usize);
    }
}
