# SSA / CFG (1.5.1)

## API
- `if_cfg` / `while_cfg` / `linear_cfg`
- `CfgBuilder`, `predecessors`, `defined_ssa`, `validate`
- **`lower_program_to_cfg(&Program) -> LoweredCfg`**
  - Flattens sequential stmts
  - if → branch shape + join phis for assigned names
  - while → loop shape + loop-carried phi candidates
  - `assigned: Vec<String>` lists mutation targets

## Next
Wire NodeIds to real IR nodes; codegen from CFG terminators to Jump/JumpIfFalse.
