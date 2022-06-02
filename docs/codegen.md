# RvJIT Code Generation Template

*This document is only for developer*

# Templates

RvJIT utilizes a template to generate codes and test suits. Additional support for extra instruction sets could be generated via template.

Templates are located at `script` folder. `generate.py` focus on generating each instruction, while `lib.py` generates test suits. Those text files are templates.

You can see that the template general follows the form of lines of instructions. Each instruction should at least describe its name and type in the last two columns. Other necessary informations include `funct3`, `funct7`, registers are also required to provided.

It's a pity that there's no standard template for every instruction set so there needs to be some modification before you could create your own template. In `generate.py`, generating functions are provided so that you could modify them to fit into your own code templates. Then, modify `lib.py` to generate the corresponding test suits. Testing is done using the `riscv-decode` crate.

You see, it's, at least, not a good idea to modify those old templates at least there's mistake.

# Tools

There're few functions you could use to generate immediate number or target type instructions. In `types.rs`, functions are provided to construct instructions with given type. All you need is to call these functions and fill in fields.