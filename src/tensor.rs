//! Tensor value type and basic operations.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Tensor {
    pub shape: Vec<usize>,
    pub data: Vec<f64>,
}

impl Tensor {
    pub fn new(shape: Vec<usize>, data: Vec<f64>) -> Result<Self, String> {
        let expected: usize = shape.iter().product();
        if data.len() != expected {
            return Err(format!("shape {:?} expects {} elements, got {}", shape, expected, data.len()));
        }
        Ok(Self { shape, data })
    }

    pub fn scalar(v: f64) -> Self {
        Self { shape: vec![], data: vec![v] }
    }

    pub fn zeros(shape: Vec<usize>) -> Self {
        let n: usize = shape.iter().product();
        Self { shape, data: vec![0.0; n] }
    }

    pub fn numel(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Display for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tensor(shape={:?}, data=...)", self.shape)
    }
}
