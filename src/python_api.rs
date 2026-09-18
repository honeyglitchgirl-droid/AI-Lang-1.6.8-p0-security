//! Optional Python bindings surface (feature-gated in full build).

#[cfg(feature = "python")]
mod py {
    use pyo3::prelude::*;
    // Full PyO3 bindings would go here when feature is enabled.
}

/// Placeholder when python feature is off.
pub fn python_available() -> bool {
    cfg!(feature = "python")
}
