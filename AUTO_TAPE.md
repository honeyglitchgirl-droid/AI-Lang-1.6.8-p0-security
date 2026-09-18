# Auto Tape (experimental runtime reverse AD)

Runtime reverse-mode autodiff via a tape of elementary operations.

**Status:** experimental. Compile-time / dual forward-mode is the supported path.

## Limits
- Soft op budget (`TAPE_OP_LIMIT`)
- Tensor ops not fully taped in all cases
- Fail-closed on budget exceed (TapeBudgetExceeded)

See `src/runtime_ad.rs` and `docs/SECURITY.md`.
