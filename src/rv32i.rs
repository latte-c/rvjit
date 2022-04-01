use crate::types::*;

pub fn lui(imm: u32, rd: u8) -> u32 {
    rv_utype(imm, rd, 0b0110111)
}

pub fn auipc(imm: u32, rd: u8) -> u32 {
    rv_utype(imm, rd, 0b0010111)
}

pub fn jal(imm: u32, rd: u8) -> u32 {
    rv_jtype(imm, rd, 0b1101111)
}

pub fn jalr(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b000, rd, 0b1100111)
}

pub fn beq(imm: u32, rs2: u8, rs1: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b000, 0b1100011)
}

pub fn bne(imm: u32, rs2: u8, rs1: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b001, 0b1100011)
}

pub fn blt(imm: u32, rs2: u8, rs1: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b100, 0b1100011)
}

pub fn bge(imm: u32, rs2: u8, rs1: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b101, 0b1100011)
}

pub fn bltu(imm: u32, rs2: u8, rs1: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b110, 0b1100011)
}

pub fn bgeu(imm: u32, rs2: u8, rs1: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b111, 0b1100011)
}

pub fn lb(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b000, rd, 0b0000011)
}

pub fn lh(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b001, rd, 0b0000011)
}

pub fn lw(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b010, rd, 0b0000011)
}

pub fn lbu(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b100, rd, 0b0000011)
}

pub fn lhu(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b101, rd, 0b0000011)
}

pub fn sb(imm: u32, rs2: u8, rs1: u8) -> u32 {
    rv_stype(imm, rs2, rs1, 0b000, 0b0100011)
}

pub fn sh(imm: u32, rs2: u8, rs1: u8) -> u32 {
    rv_stype(imm, rs2, rs1, 0b001, 0b0100011)
}

pub fn sw(imm: u32, rs2: u8, rs1: u8) -> u32 {
    rv_stype(imm, rs2, rs1, 0b010, 0b0100011)
}

pub fn addi(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b000, rd, 0b0010011)
}

pub fn slti(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b010, rd, 0b0010011)
}

pub fn sltiu(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b011, rd, 0b0010011)
}

pub fn xori(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b100, rd, 0b0010011)
}

pub fn ori(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b110, rd, 0b0010011)
}

pub fn andi(imm: u32, rs1: u8, rd: u8) -> u32 {
    rv_itype(imm, rs1, 0b111, rd, 0b0010011)
}

pub fn slli(shamt: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, shamt, rs1, 0b001, rd, 0b0010011)
}

pub fn srli(shamt: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, shamt, rs1, 0b101, rd, 0b0010011)
}

pub fn srai(shamt: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0100000, shamt, rs1, 0b101, rd, 0b0010011)
}

pub fn add(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b000, rd, 0b0110011)
}

pub fn sub(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0100000, rs2, rs1, 0b000, rd, 0b0110011)
}

pub fn sll(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b001, rd, 0b0110011)
}

pub fn slt(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b010, rd, 0b0110011)
}

pub fn sltu(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b011, rd, 0b0110011)
}

pub fn xor(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b100, rd, 0b0110011)
}

pub fn srl(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b101, rd, 0b0110011)
}

pub fn sra(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0100000, rs2, rs1, 0b101, rd, 0b0110011)
}

pub fn or(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b110, rd, 0b0110011)
}

pub fn and(rs2: u8, rs1: u8, rd: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b111, rd, 0b0110011)
}

pub fn ecall() -> u32 {
    rv_itype(0, 0, 0b001, 0, 0b1110011)
}
pub fn ebreak() -> u32 {
    rv_itype(0, 0, 0b000, 0, 0b1110011)
}
