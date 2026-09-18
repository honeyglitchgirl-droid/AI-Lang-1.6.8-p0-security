# AI-Lang roadmap (current: 1.6.x)

See **RELEASE_STATUS.md** for the authoritative feature matrix.

## Done (1.6.x)
- List pipeline, easy-run CLI (`-e`, REPL)
- VM hardening: local/list limits, jump boundaries, import path containment
- Compile-time AD; runtime tape marked experimental

## Next
1. Unify CFG path with main IR codegen; fix if/while optimizer NodeId edge
2. CI: fmt, clippy -D warnings, full test matrix, cargo audit
3. Classify remaining non-test unwrap/expect (panic policy CI)
4. Runtime AD Option B only if product requires it; else keep experimental
5. Real CUDA only with cudarc + device tests

## Later
Python wheels, package registry, formal verification of selected invariants
