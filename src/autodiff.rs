//! Compile-time and forward-mode automatic differentiation support.
//!
//! Dual numbers for runtime forward-mode; gradient tape hooks for reverse mode
//! (experimental).

use crate::tensor::Tensor;
use crate::value::Value;

/// Forward-mode dual number: primal + tangent.
#[derive(Debug, Clone, PartialEq)]
pub struct Dual {
    pub primal: f64,
    pub tangent: f64,
}

impl Dual {
    pub fn new(primal: f64, tangent: f64) -> Self {
        Self { primal, tangent }
    }

    pub fn from_primal(p: f64) -> Self {
        Self {
            primal: p,
            tangent: 0.0,
        }
    }

    pub fn seed(p: f64) -> Self {
        Self {
            primal: p,
            tangent: 1.0,
        }
    }
}

impl std::ops::Add for Dual {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            primal: self.primal + rhs.primal,
            tangent: self.tangent + rhs.tangent,
        }
    }
}

impl std::ops::Sub for Dual {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            primal: self.primal - rhs.primal,
            tangent: self.tangent - rhs.tangent,
        }
    }
}

impl std::ops::Mul for Dual {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            primal: self.primal * rhs.primal,
            tangent: self.primal * rhs.tangent + self.tangent * rhs.primal,
        }
    }
}

impl std::ops::Div for Dual {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let p = self.primal / rhs.primal;
        let t = (self.tangent * rhs.primal - self.primal * rhs.tangent) / (rhs.primal * rhs.primal);
        Self { primal: p, tangent: t }
    }
}

/// Extract primal / tangent from a Value if it holds a Dual.
pub fn primal(v: &Value) -> Option<f64> {
    // Full implementation matches on Value::Dual / Value::Tensor etc.
    let _ = v;
    None
}

pub fn tangent(v: &Value) -> Option<f64> {
    let _ = v;
    None
}

/// Compute gradients of a scalar loss w.r.t. marked parameters (compile-time path).
pub fn compute_gradients(_program: &crate::ast::Program) -> Result<(), crate::error::CompileError> {
    // Placeholder for the reverse-mode / dual lowering pass.
    Ok(())
}
