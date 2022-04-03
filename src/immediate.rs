// generate immediate number for different types of instructions

pub(crate) fn immediate_i(imm: u32) -> u32 {
    let imm_low = imm & 0b111111111111;
    imm_low << 20
}

pub(crate) fn immediate_s(imm: u32) -> u32 {
    let imm_high: u32 = (imm & 0b111111100000) >> 5;
    let imm_low: u32 = imm & 0b11111;
    imm_high << 25 | imm_low << 7
}

pub(crate) fn immediate_b(imm: u32) -> u32 {
    let imm_a = (imm & 0b1000000000000) >> 12;
    let imm_b = (imm & 0b0011111100000) >> 5;
    let imm_c = (imm & 0b0000000011110) >> 1;
    let imm_d = (imm & 0b0100000000000) >> 11;
    imm_a << 31 | imm_b << 25 | imm_c << 8 | imm_d << 7
}

pub(crate) fn immediate_u(imm: u32) -> u32 {
    let imm_high: u32 = (imm & 0b11111111111111111111000000000000) >> 12;
    imm_high << 12
}

pub(crate) fn immediate_j(imm: u32) -> u32 {
    let imm_a = (imm & 0b100000000000000000000) >> 20;
    let imm_b = (imm & 0b000000000011111111110) >> 1;
    let imm_c = (imm & 0b000000000100000000000) >> 11;
    let imm_d = (imm & 0b011111111000000000000) >> 12;
    imm_a << 31 | imm_b << 21 | imm_c << 20 | imm_d << 12
}
