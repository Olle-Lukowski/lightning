use core::arch::asm;

use crate::hal::arch::riscv::csr::{Misa, Satp};

use super::Environment;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Mode {
    #[default]
    Machine,
    Supervisor,
    User,
}

pub struct ExecutionEnvironment {
    kernel: Mode,
    user: Mode,
}

impl ExecutionEnvironment {
    /// Create AND activate the default [`ExecutionEnvironment`].
    ///
    /// Will prefer S-mode for kernel space, falling back to M-mode if unavailable.
    /// Will prefer U-mode for user space, falling back to M-mode if unavailable.
    ///
    /// # Safety
    /// MUST be called in M-mode with no virtual memory set up.
    pub unsafe fn new() -> ExecutionEnvironment {
        let extensions = unsafe { Misa::read() }.extensions();

        let kernel = if extensions.s() {
            Mode::Supervisor
        } else {
            Mode::Machine
        };

        if kernel == Mode::Supervisor {
            // switch to supervisor mode

            // First, ensure we don't have memory protection or translation
            // SAFETY: Caller guarantees the safety contract is upheld
            unsafe { Satp::new_with_raw_value(0).write() };
        }

        let user = if extensions.u() {
            Mode::User
        } else {
            Mode::Machine
        };

        ExecutionEnvironment { kernel, user }
    }
}

impl Environment for ExecutionEnvironment {}
