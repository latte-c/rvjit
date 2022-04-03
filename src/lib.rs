mod immediate;
pub mod rv32i;
pub mod rv64i;
mod types;

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

#[cfg(test)]
mod tests {
    extern crate riscv_decode as dasm;
    use crate::{rv32i::*, *};

    #[test]
    fn lui_test() {
        let inst = lui(A0, 934531072);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Lui(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.imm(), 934531072 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn auipc_test() {
        let inst = auipc(A0, 934531072);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Auipc(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.imm(), 934531072 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn jal_test() {
        let inst = jal(A0, 1479496);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Jal(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.imm(), 1479496 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn jalr_test() {
        let inst = jalr(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Jalr(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn beq_test() {
        let inst = beq(4858, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Beq(i) = decoded {
            assert_eq!(i.imm(), 4858 as u32, "imm test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn bne_test() {
        let inst = bne(4858, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Bne(i) = decoded {
            assert_eq!(i.imm(), 4858 as u32, "imm test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn blt_test() {
        let inst = blt(4858, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Blt(i) = decoded {
            assert_eq!(i.imm(), 4858 as u32, "imm test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn bge_test() {
        let inst = bge(4858, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Bge(i) = decoded {
            assert_eq!(i.imm(), 4858 as u32, "imm test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn bltu_test() {
        let inst = bltu(4858, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Bltu(i) = decoded {
            assert_eq!(i.imm(), 4858 as u32, "imm test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn bgeu_test() {
        let inst = bgeu(4858, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Bgeu(i) = decoded {
            assert_eq!(i.imm(), 4858 as u32, "imm test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn lb_test() {
        let inst = lb(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Lb(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn lh_test() {
        let inst = lh(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Lh(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn lw_test() {
        let inst = lw(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Lw(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn lbu_test() {
        let inst = lbu(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Lbu(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn lhu_test() {
        let inst = lhu(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Lhu(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn sb_test() {
        let inst = sb(A1, A2, 3434);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Sb(i) = decoded {
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
            assert_eq!(i.imm(), 3434 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn sh_test() {
        let inst = sh(A1, A2, 3434);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Sh(i) = decoded {
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
            assert_eq!(i.imm(), 3434 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn sw_test() {
        let inst = sw(A1, A2, 3434);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Sw(i) = decoded {
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
            assert_eq!(i.imm(), 3434 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn addi_test() {
        let inst = addi(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Addi(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn slti_test() {
        let inst = slti(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Slti(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn sltiu_test() {
        let inst = sltiu(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Sltiu(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn xori_test() {
        let inst = xori(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Xori(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn ori_test() {
        let inst = ori(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Ori(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn andi_test() {
        let inst = andi(A0, A1, 3375);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Andi(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.imm(), 3375 as u32, "imm test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn slli32_test() {
        let inst = slli32(A0, A1, 28);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Slli(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.shamt(), 28 as u32, "shamt test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn srli32_test() {
        let inst = srli32(A0, A1, 28);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Srli(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.shamt(), 28 as u32, "shamt test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn srai32_test() {
        let inst = srai32(A0, A1, 28);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Srai(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.shamt(), 28 as u32, "shamt test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn add_test() {
        let inst = add(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Add(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn sub_test() {
        let inst = sub(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Sub(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn sll_test() {
        let inst = sll(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Sll(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn slt_test() {
        let inst = slt(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Slt(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn sltu_test() {
        let inst = sltu(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Sltu(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn xor_test() {
        let inst = xor(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Xor(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn srl_test() {
        let inst = srl(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Srl(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn sra_test() {
        let inst = sra(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Sra(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn or_test() {
        let inst = or(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::Or(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn and_test() {
        let inst = and(A0, A1, A2);
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if let dasm::Instruction::And(i) = decoded {
            assert_eq!(i.rd(), A0 as u32, "rd test failed");
            assert_eq!(i.rs1(), A1 as u32, "rs1 test failed");
            assert_eq!(i.rs2(), A2 as u32, "rs2 test failed");
        } else {
            panic!("opcode test failed");
        }
    }

    #[test]
    fn ecall_test() {
        let inst = ecall();
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if decoded != dasm::Instruction::Ecall {
            panic!("opcode test failed, got {:?}", decoded);
        }
    }

    #[test]
    fn ebreak_test() {
        let inst = ebreak();
        let decoded = dasm::decode(inst).expect("invalid instruction encoding");
        if decoded != dasm::Instruction::Ebreak {
            panic!("opcode test failed, got {:?}", decoded);
        }
    }
}
