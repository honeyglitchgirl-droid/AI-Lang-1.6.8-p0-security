# AI-Lang Python bindings

## Pure Rust embedding (always available)

```rust
use ai_lang::run_source;
let out = run_source("print 1 + 2;").unwrap();
```

## PyO3 extension (`--features python`)

```toml
# Cargo.toml already has:
# python = ["dep:pyo3"]
```

```bash
# Requires network for pyo3 the first time
cargo build --features python
# or with maturin:
maturin develop --features python
```

```python
import ai_lang
print(ai_lang.run("print 1 + 2;"))   # "3\n"
bc = ai_lang.compile("print 42;")    # bytes
```

The `cdylib` crate type is enabled so the shared library can be loaded by Python.
