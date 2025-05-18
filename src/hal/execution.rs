#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;

pub trait Environment: Send + Sync {}

impl Environment for () {}

pub unsafe fn setup_environment() -> impl Environment {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        riscv::ExecutionEnvironment::new()
    }
}
