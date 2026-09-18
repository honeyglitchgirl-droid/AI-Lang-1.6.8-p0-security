# AI-Lang 1.6.4 — Security & production hardening

## Hardened in this release
1. **OP_STORE** — local index capped at `LOCAL_COUNT_LIMIT` (65_536).
2. **Jump/Call targets** — instruction-boundary validation.
3. **ListPush** — `LIST_ELEMENT_LIMIT` (1_048_576).
4. **String pool** — `StringPoolOutOfBounds`.
5. **Import paths** — no `..` in specs; must stay under root.
6. **Runtime AD tape** — soft op budget; **experimental** (tensor ops not fully taped). Compile-time AD is the supported path.

## Resource limits
STACK_DEPTH_LIMIT, CALL_DEPTH_LIMIT, STEP_BUDGET, MAX_TENSOR_ELEMENTS,
LOCAL_COUNT_LIMIT, LIST_ELEMENT_LIMIT, TAPE_OP_LIMIT.

## Not a sandbox
Resource limits are **not** a security boundary. Untrusted AI-Lang programs
must not run in privileged host processes without an external isolation layer.
