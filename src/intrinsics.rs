//! Built-in functions and intrinsics (aether, damp, pulse, list ops, etc.).

use crate::error::VmError;
use crate::value::Value;

pub fn aether(x: f64) -> f64 {
    // x * (1 + tanh(x)) / (1 + |x|)
    let t = x.tanh();
    x * (1.0 + t) / (1.0 + x.abs())
}

pub fn damp(g: f64) -> f64 {
    // g / (1 + |g|)
    g / (1.0 + g.abs())
}

pub fn pulse(x: f64) -> f64 {
    // small sin-modulated decay
    0.01 * (x * 3.14159).sin() * (-x.abs()).exp()
}

pub fn call_intrinsic(name: &str, args: &[Value]) -> Result<Value, VmError> {
    match name {
        "aether" => {
            let x = args.get(0).and_then(|v| v.as_f64()).ok_or(VmError::Other("aether expects number".into()))?;
            Ok(Value::Float(aether(x)))
        }
        "damp" => {
            let g = args.get(0).and_then(|v| v.as_f64()).ok_or(VmError::Other("damp expects number".into()))?;
            Ok(Value::Float(damp(g)))
        }
        "pulse" => {
            let x = args.get(0).and_then(|v| v.as_f64()).ok_or(VmError::Other("pulse expects number".into()))?;
            Ok(Value::Float(pulse(x)))
        }
        "list_len" => {
            if let Some(Value::List(xs)) = args.get(0) {
                Ok(Value::Int(xs.len() as i64))
            } else {
                Err(VmError::Other("list_len expects list".into()))
            }
        }
        "primal" | "tangent" => {
            // Handled specially in VM for Dual values
            Err(VmError::Other(format!("{name} handled by VM")))
        }
        _ => Err(VmError::Other(format!("unknown intrinsic: {name}"))),
    }
}
