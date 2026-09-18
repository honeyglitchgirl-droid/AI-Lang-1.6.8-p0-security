# Security audit comparison through 1.6.8

| ID | Finding | 1.6.7 | 1.6.8 |
|----|---------|-------|-------|
| F-01 | MatMul pre-alloc | FIXED | FIXED |
| F-02 | Public Tensor / BLAS | mitigated | + tangent length checks before dgemm |
| F-03 | CI audit fail-open | FIXED | FIXED |
| F-04 | Action pins | partial | partial |
| F-05 | Tape fail-open | silent drop | **FIXED fail-closed** → TapeBudgetExceeded |
| F-07 / N-02 | Input quota | declared only | **FIXED** byte-by-byte bound |
| N-01 | Verifier not integrated | module only | **FIXED** — `VerifiedBytecode::verify` at `run_impl` entry; validates JUMP/CALL targets |
| N-04 | Tangent length | open | **FIXED** in matmul |
| F-02 full privacy | public fields | open | still public (API break deferred) |
| F-06 TOCTOU | open | open |
| F-10 LOCK enforce | open | open |

## Still not production-hardened
- Tensor fields not private
- Full stack-height analysis in verifier
- Holistic memory budget
- Adversarial fuzz corpus

Verdict: **IMPROVED**; still **not** a hostile-code sandbox.
