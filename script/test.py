import os

RS1 = 'A1'
RS2 = 'A2'
RD = 'A0'
SHAMT = 0b11100
IMM_I = 0b110100101111  # 12 bits
IMM_S = 0b110101101010  # 12 bits
IMM_B = 0b1001011111010  # 13 bits
IMM_U = 934534535 & 0xfffff000
IMM_J = 0b101101001001101001000  # 21 bits


def gen_rv32i_inst(encode: str) -> str:
    splited = encode.strip().split(' ')
    inst_name = splited[-1]
    inst_type = splited[-2]
    upper_inst_name = inst_name[0].upper() + inst_name[1:]

    if inst_name in ['csrrw', 'csrrs', 'csrrc', 'csrrwi', 'csrrsi', 'csrrci']:
        return ""

    if inst_name in ['fence', 'fence.i']:
        return ""

    if inst_name in ['ecall', 'ebreak']:
        return f"""
        #[test]
        fn {inst_name}_test() {{
            let inst = {inst_name}();
            let decoded = dasm::decode(inst).expect("invalid instruction encoding");
            if decoded != dasm::Instruction::{upper_inst_name} {{
                panic!("opcode test failed, got {{:?}}", decoded);
            }}
        }}
        """
    else:
        if inst_type == 'U':
            param = [
                ("rd", RD),
                ("imm", IMM_U)
            ]
        elif inst_type == 'J':
            param = [
                ("rd", RD),
                ("imm", IMM_J)
            ]
        elif inst_type == 'I':
            if inst_name in ['slli', 'srli', 'srai']:
                param = [ 
                    ("rd", RD),
                    ("rs1", RS1),
                    ("shamt", SHAMT)
                ]
            else:
                param = [
                    ("rd", RD),
                    ("rs1", RS1),
                    ("imm", IMM_I)
                ]
        elif inst_type == 'B':
            param = [
                ("imm", IMM_B),
                ("rs1", RS1),
                ("rs2", RS2)
            ]
        elif inst_type == 'R':
            param = [
                ("rd", RD),
                ("rs1", RS1),
                ("rs2", RS2)
            ]
        elif inst_type == 'S':
            param = [
                ("rs1", RS1),
                ("rs2", RS2),
                ("imm", IMM_S)
            ]
        else:
            raise Exception(f'unknown instruction type {inst_type}')

    inst_param = ', '.join(map(lambda x: str(x[1]), param))
    assertions = ';\n'.join(
        map(lambda x: f'assert_eq!(i.{x[0]}(), {x[1]} as u32, "{x[0]} test failed");', param)
    )

    return f"""
    #[test]
    fn {inst_name}_test() {{
        let inst = {inst_name}({inst_param});
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::{upper_inst_name}(i) = decoded {{
            {assertions}
        }} else {{
            panic!("opcode test failed");
        }}
    }}
    """


with open('rv32i.txt', 'r') as f:
    with open('../src/lib.rs', 'w') as of:
        of.write("""mod immediate;
        pub mod register;
        pub mod rv32i;
        mod types;
        
        #[cfg(test)]
        mod tests {
            extern crate riscv_decode as dasm;
            use crate::{register::*, rv32i::*};
        """)
        for l in f.readlines():
            inst = gen_rv32i_inst(l)
            of.write(inst + '\n\n')
        of.write("}\n")

os.system('rustfmt ../src/lib.rs')
