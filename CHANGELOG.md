## 1.6.8
- P0: verifier integrated + control-flow targets
- P0: input quota enforced
- P0: tape budget fail-closed
- P0: tangent length checks

## 1.6.7
- F-01 matmul size check before alloc
- F-07 I/O quotas
- F-03 CI audit fail-closed
- bytecode_verify module
- BLAS preconditions + SAFETY
- docs/SECURITY_COMPARE.md

# Changelog

## 1.6.6
- Multi-platform GitHub Actions CI (ubuntu/macos/windows): fmt, check, test, clippy, release, examples
- `docs/PANIC_CLASSIFICATION.md` — systematic non-test unwrap/expect classification
- `scripts/gen_sbom.py` + `sbom.json` (CycloneDX-style from Cargo.lock)
- `scripts/check_unwraps.sh` inventory helper
- CI audit job (cargo-audit when available)
- Driver path handling without unwrap; `#![allow(missing_docs)]` until docs filled

## 1.6.5
- RELEASE_STATUS + VERIFICATION evidence; hello_language string pool fix
