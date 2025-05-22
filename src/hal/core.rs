use crate::handle_trap;

use super::trap::{Trap, TrapHandler};

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

/// Check whether the currently running core is the primary one of the system.
pub fn is_primary_core() -> bool {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv::is_primary_hart()
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
type Env = super::execution::riscv::ExecutionEnvironment;

/// The state of a core.
pub struct CoreState {
    /// The trap handler for the core.
    pub trap_handler: TrapHandler,
    /// The execution environment of the core.
    pub env: Env,
}

/// A handle to a core, ensures the [`CoreState`] is loaded during its lifetime.
pub struct Core<'core> {
    /// The state of the core.
    pub state: &'core CoreState,
}

impl Drop for Core<'_> {
    fn drop(&mut self) {
        use ::riscv::register::mscratch;

        let state = unsafe { &*(mscratch::read() as *mut CoreState) };
        unsafe {
            mscratch::write(0);
        }

        if state.env.kernel == super::execution::riscv::Mode::Supervisor {
            unsafe {
                use ::riscv::register::sscratch;
                sscratch::write(0);
            }
        }
    }
}

impl CoreState {
    pub fn new() -> CoreState {
        CoreState {
            trap_handler: handle_trap,
            env: Env::new(),
        }
    }

    pub fn load<'core>(&'core self) -> Core<'core> {
        let raw = self as *const _ as usize;

        // SAFETY: The lifetime ensures this will never point to an invalid state
        unsafe {
            use ::riscv::register::mscratch;
            mscratch::write(raw);
        }

        if self.env.kernel == super::execution::riscv::Mode::Supervisor {
            // SAFETY: The lifetime ensures this will never point to an invalid state
            unsafe {
                use ::riscv::register::sscratch;
                sscratch::write(raw);
            }
        }

        Core { state: self }
    }

    pub fn handle_trap(&self, trap: Trap) {
        (self.trap_handler)(trap)
    }
}
