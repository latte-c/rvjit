# RvJIT Document

This is the main document of project `RvJIT`, an easy to use RISC-V 32/64 assembler.

* User
    * [Usage and Convention](#usage-and-convention)
    * [Constant Values](#constant-values)
    * [Miscellaneous](#miscellaneous)

* Developer
    * [Code Generation](./codegen.md)

## Usage and Convention

* **Usage**

Instruction assembler is provided in the following form:

```
rv{width}{instruction-set}::{instruction-name}({parameters})
```

where `width` could be `32` and `64` base on your needs. `instruction-set` could be `i` or `m` currently. `instruction-name` is the name of the instruction (officially).

There're some exceptions, though. In 64 bits mode, shifting operations could use more `shamt`, thus, both 32 bits and 64 bits operation should be provided but with the same name. In rust, function overloading is not provided, so only way out is to change the name. Solution is, 64 bits operation is post-fixed with `64` and 32 bits operation with the original name. Changed operations are: `slli64`, `srli64`, `srai64`.

* **Convention**

Parameters are all unsigned integers with different length. Registers are `u8`, immediate numbers are `u32`. Note that some groups of operations require to prune the lower bits of the immmediate number and **we do exactly the same**, so we won't do some shiftings or things like that.

The order of parameters try to keep the same with assembly, thus, in the order of `rd, rs1, rs2`. `shamt` is considered as immediate number, thus in the end.

## Constant Values

Constant values of registers are also provided. Two sets of naming are provided, one is `x0` to `x31`, accessed by `rvjit::X{id}`, another set is the assembly name of registers, just use the upper case of the name would be fine.

## Miscellaneous

`no-std` feature is available for this crate, as it could be used in conditions where only raw-board is provided.

Do not import every thing for `rv32*` and `rv64*` at the same time in order to avoid naming conflict.