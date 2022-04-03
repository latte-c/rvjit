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


def gen_rv32i_inst_test(encode: str) -> str:
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
                inst_name = inst_name + '32'
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

REGS = """
// general purpose registers
// from x0 to x31, 32 integer registers, in u8
pub const X0: u8 = 0;
pub const X1: u8 = 1;
pub const X2: u8 = 2;
pub const X3: u8 = 3;
pub const X4: u8 = 4;
pub const X5: u8 = 5;
pub const X6: u8 = 6;
pub const X7: u8 = 7;
pub const X8: u8 = 8;
pub const X9: u8 = 9;
pub const X10: u8 = 10;
pub const X11: u8 = 11;
pub const X12: u8 = 12;
pub const X13: u8 = 13;
pub const X14: u8 = 14;
pub const X15: u8 = 15;
pub const X16: u8 = 16;
pub const X17: u8 = 17;
pub const X18: u8 = 18;
pub const X19: u8 = 19;
pub const X20: u8 = 20;
pub const X21: u8 = 21;
pub const X22: u8 = 22;
pub const X23: u8 = 23;
pub const X24: u8 = 24;
pub const X25: u8 = 25;
pub const X26: u8 = 26;
pub const X27: u8 = 27;
pub const X28: u8 = 28;
pub const X29: u8 = 29;
pub const X30: u8 = 30;
pub const X31: u8 = 31;

// alias registers
pub const ZERO: u8 = X0;
pub const RA: u8 = X1;
pub const SP: u8 = X2;
pub const GP: u8 = X3;
pub const TP: u8 = X4;
pub const T0: u8 = X5;
pub const T1: u8 = X6;
pub const T2: u8 = X7;
pub const S0: u8 = X8;
pub const FP: u8 = X8;
pub const S1: u8 = X9;
pub const A0: u8 = X10;
pub const A1: u8 = X11;
pub const A2: u8 = X12;
pub const A3: u8 = X13;
pub const A4: u8 = X14;
pub const A5: u8 = X15;
pub const A6: u8 = X16;
pub const A7: u8 = X17;
pub const S2: u8 = X18;
pub const S3: u8 = X19;
pub const S4: u8 = X20;
pub const S5: u8 = X21;
pub const S6: u8 = X22;
pub const S7: u8 = X23;
pub const S8: u8 = X24;
pub const S9: u8 = X25;
pub const S10: u8 = X26;
pub const S11: u8 = X27;
pub const T3: u8 = X28;
pub const T4: u8 = X29;
pub const T5: u8 = X30;
pub const T6: u8 = X31;
"""


with open('rv32i.txt', 'r') as f:
    with open('../src/lib.rs', 'w') as of:
        of.write(f"""mod immediate;
        pub mod rv32i;
        pub mod rv64i;
        mod types;

        {REGS}
        
        #[cfg(test)]
        mod tests {{
            extern crate riscv_decode as dasm;
            use crate::{{rv32i::*, *}};
        """)
        for l in f.readlines():
            inst = gen_rv32i_inst_test(l)
            of.write(inst + '\n\n')
        of.write("}\n")

os.system('rustfmt ../src/lib.rs')
