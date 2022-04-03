use crate::types::*;

pub fn lwu(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b110, rd, 0b0000011)
}

pub fn ld(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b011, rd, 0b0000011)
}

pub fn sd(rs1: u8, rs2: u8, imm: u32) -> u32 {
    rv_stype(imm, rs2, rs1, 0b011, 0b0100011)
}

pub fn slli64(rd: u8, rs1: u8, shamt: u8) -> u32 {
    rv_itype(
        0b000000000000 | (shamt as u32 & 0b111111),
        rs1,
        0b001,
        rd,
        0b0010011,
    )
}

pub fn srli64(rd: u8, rs1: u8, shamt: u8) -> u32 {
    rv_itype(
        0b000000000000 | (shamt as u32 & 0b111111),
        rs1,
        0b101,
        rd,
        0b0010011,
    )
}

pub fn srai64(rd: u8, rs1: u8, shamt: u8) -> u32 {
    rv_itype(
        0b010000000000 | (shamt as u32 & 0b111111),
        rs1,
        0b101,
        rd,
        0b0010011,
    )
}

pub fn addiw(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b000, rd, 0b0011011)
}

pub fn slliw(rd: u8, rs1: u8, shamt: u8) -> u32 {
    rv_rtype(0b0000000, shamt, rs1, 0b001, rd, 0b0011011)
}

pub fn srliw(rd: u8, rs1: u8, shamt: u8) -> u32 {
    rv_rtype(0b0000000, shamt, rs1, 0b101, rd, 0b0011011)
}

pub fn sraiw(rd: u8, rs1: u8, shamt: u8) -> u32 {
    rv_rtype(0b0100000, shamt, rs1, 0b101, rd, 0b0011011)
}

pub fn addw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b000, rd, 0b0111011)
}

pub fn subw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0100000, rs2, rs1, 0b000, rd, 0b0111011)
}

pub fn sllw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b001, rd, 0b0111011)
}

pub fn srlw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b101, rd, 0b0111011)
}

pub fn sraw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0100000, rs2, rs1, 0b101, rd, 0b0111011)
}
