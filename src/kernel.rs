//! Device kernels and hardware abstraction.
//!
//! HostCPU, VectorUnit, and Accelerator kernel stubs.
//! Real CUDA is architecture-only in this release.

use crate::tensor::Tensor;
use crate::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Device {
    HostCPU,
    VectorUnit,
    Accelerator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwarePolicy {
    Auto,
    FallbackToHost,
    Strict,
}

pub struct DriverManager {
    policy: HardwarePolicy,
    accelerator_available: bool,
}

impl DriverManager {
    pub fn with_all_devices() -> Self {
        let accelerator_available = std::env::var("AI_LANG_ACCELERATOR").is_ok();
        Self {
            policy: HardwarePolicy::Auto,
            accelerator_available,
        }
    }

    pub fn select_device(&self, _workload_size: usize) -> Device {
        match self.policy {
            HardwarePolicy::Auto => {
                if self.accelerator_available {
                    Device::Accelerator
                } else {
                    Device::HostCPU
                }
            }
            HardwarePolicy::FallbackToHost => Device::HostCPU,
            HardwarePolicy::Strict => {
                if self.accelerator_available {
                    Device::Accelerator
                } else {
                    Device::HostCPU
                }
            }
        }
    }
}

/// Host-side matmul / elementwise kernels used by the VM.
pub fn host_matmul(a: &Tensor, b: &Tensor) -> Result<Tensor, String> {
    // Shape checks and allocation happen before call in full source.
    let _ = (a, b);
    Err("host_matmul: full implementation in original source".into())
}
