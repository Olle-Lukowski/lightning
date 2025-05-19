use core::arch::asm;

use riscv::register::{
    Permission, Range,
    medeleg::{self, Medeleg},
    mepc,
    mideleg::{self, Mideleg},
    misa,
    mstatus::{self, MPP},
    pmpaddr0, pmpcfg0,
    satp::{self, Satp},
};

use crate::main;

use super::Environment;

/// A privilege mode of a hart
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Mode {
    #[default]
    Machine,
    Supervisor,
    User,
}

/// The [`Mode`]s that will be used for kernel and user-space.
pub struct ExecutionEnvironment {
    pub kernel: Mode,
    pub user: Mode,
}

impl ExecutionEnvironment {
    /// Create the default [`ExecutionEnvironment`].
    ///
    /// Will prefer S-mode for kernel space, falling back to M-mode if unavailable.
    /// Will prefer U-mode for user space, falling back to M-mode if unavailable.
    ///
    pub fn new() -> ExecutionEnvironment {
        let isa = misa::read();
        let kernel = if isa.has_extension('S') {
            Mode::Supervisor
        } else {
            Mode::Machine
        };

        let user = if isa.has_extension('U') {
            Mode::User
        } else {
            Mode::Machine
        };

        ExecutionEnvironment { kernel, user }
    }
}

impl Environment for ExecutionEnvironment {
    /// Activate the execution environment.
    ///
    /// # Safety
    ///
    /// MUST be called in M-mode, with no virtual memory set up.
    /// A valid M-mode trap handler must be active.
    ///
    unsafe fn activate(&self) {
        if self.kernel == Mode::Supervisor {
            // switch to supervisor mode

            // SAFETY: Caller guarantees the safety contract is upheld
            unsafe {
                // First, ensure we don't have memory protection or translation
                satp::write(Satp::from_bits(0));

                // Then, delegate all exceptions we can to supervisor mode.
                medeleg::write(Medeleg::from_bits(!0));

                // Also delegate all interrupts to supervisor mode.
                mideleg::write(Mideleg::from_bits(!0));

                // Allow supervisor mode to access all physical memory
                pmpcfg0::set_pmp(0, Range::NAPOT, Permission::RWX, false);
                pmpaddr0::write(!0);

                // Everything is set up, time for S-mode!
                mepc::write(main as usize);
                mstatus::set_mpp(MPP::Supervisor);

                asm!("la ra, 1f", "mret", "1:");
            }
        }
    }
}
