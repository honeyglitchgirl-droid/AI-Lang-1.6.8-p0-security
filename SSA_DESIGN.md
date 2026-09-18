# SSA / CFG design (path to production loops)

## Goal
Represent mutation and loops so optimization and autodiff can see control-flow joins.

## Target form
```
entry:
  i0 = 0
  jump loop
loop:
  i = phi(i0, i_next)
  cond = i < n
  branch cond, body, exit
body:
  i_next = i + 1
  jump loop
exit:
  ...
```

## Migration plan
1. Keep current graph IR for pure expressions.
2. Lower `while`/`for`/`assign` to basic blocks + phi in a new `CfgIr` pass.
3. Run existing optimizer on pure subgraphs; add LICM / DSE on CFG.
4. Dual/forward AD remains value-based; reverse AD later records across blocks.

## Status
Design accepted. Implementation is the next major IR milestone after Dual tensors.
