# Memory safety notes

Rust implementation aims for safe defaults.
VM has stack / local / list / step budgets.
Tensor allocations checked before large ops (e.g. matmul).

Not a sandbox for untrusted programs.
