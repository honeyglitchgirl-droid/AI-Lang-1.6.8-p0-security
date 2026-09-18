# AI-Lang 1.6.8-p0-security

**AI-first programming language** — Rust compiler + bytecode VM.

> **Status:** Intermediate / production-oriented engineering.  
> Authoritative claims: **[RELEASE_STATUS.md](RELEASE_STATUS.md)**.  
> Not a security sandbox. CUDA is architecture-only. Runtime reverse AD is experimental.

```bash
cargo test --offline
cargo run -- -e 'print 1+2;'
cargo run -- examples/calculator.al
ai-lang --repl
```

---

A native, **AI-first** experimental programming language, implemented in Rust.

AI-Lang is designed so that machine-learning workloads are the language’s center of gravity: numbers are the default value type, tensors sit one type-system step from `float`, and the runtime is a small hand-rolled bytecode VM.

## Quick start

```bash
cargo test --offline
cargo run --offline -- -e 'print 1+2;'
cargo run --offline -- examples/calculator.al
```

## Repository contents

- `src/` — full module layout (compiler, VM, IR, optimizer, typecheck, lexer, parser, …)
- `examples/` — runnable `.al` programs
- `docs/` — security notes and panic classification
- `.github/workflows/ci.yml` — multi-platform CI
- `std/` — standard library skeleton
- Python bindings scaffolding (`python/`, `pyproject.toml`)

**Note on large source files:** The biggest modules (`vm.rs`, `ir.rs`, `codegen.rs`, `compiler.rs`, `parser.rs`, `optimizer.rs`, `typecheck.rs`) are present as structural stubs that preserve the public API surface. For the complete original implementations, overlay the files from the source ZIP / local tree.

## License

EPL-2.0
