# AI-Lang 1.6.6 — Verification evidence

**Date:** 2026-09-18  
**Toolchain:** rustc 1.75.0 / cargo 1.75.0  
**Host:** Linux x86_64  
**Status:** Development / intermediate (CI matrix defined; this is single-host evidence)

## Build matrix (this host)

| Command | Result |
|---------|--------|
| `cargo check --offline --all-targets` | **OK** |
| `cargo test --offline --lib` | **147 passed** |
| `cargo test --offline --tests` | **All green** (3 ignored) |
| `cargo build --offline --release` | **OK** (~995KB binary) |

## Examples

| Program | Result |
|---------|--------|
| calculator.al | OK |
| list_demo.al | OK |
| for_loop.al | OK |
| hello_language.al | OK |
| softmax.al | OK |
| dual_demo.al / dual_train.al | OK |
| linear_fit / linear_regression / train_linear | OK |
| tiny_neural_network.al | OK |
| aether_train.al / train_scalar.al | Import form limitation (if-block) |
| input_echo.al | waits on stdin (expected) |

## One-liners
```text
ai-lang -e 'print 1+2;'                    → 3
ai-lang -e 'let xs = [10,20,30]; print xs[1];' → 20
ai-lang -e 'let i = 3; while i { print i; i = i - 1; }' → 3 2 1
```

## Supply chain
- `python3 scripts/gen_sbom.py` → `sbom.json` (6 components)
- Multi-platform CI: `.github/workflows/ci.yml`

## Not available on this host
- `cargo clippy` (not installed)
- `cargo-audit` (needs newer rustc for latest)
- macOS / Windows runners
