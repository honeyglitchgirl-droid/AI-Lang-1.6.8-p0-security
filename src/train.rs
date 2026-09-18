//! Training helpers and gradient update primitives.

use crate::error::VmError;
use crate::value::Value;

/// Apply a simple gradient step: param = param - lr * grad.
/// Used by the `gradient_update` language construct.
pub fn gradient_step(param: &mut Value, grad: &Value, lr: f64) -> Result<(), VmError> {
    // Implementation depends on Value representation (scalar / tensor).
    // Full version in the original source handles Dual and Tensor cases.
    let _ = (param, grad, lr);
    Ok(())
}
