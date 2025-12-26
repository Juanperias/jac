use crate::{backend::RawIns, frontend::parser::Register};

pub const fn ldi(reg: u8, imm: u16) -> RawIns {
    const OPCODE: u16 = 0b1110;

    let opcode =
        (OPCODE << 12) | ((imm as u16 & 0xF0) << 4) | ((reg) << 4) as u16 | (imm as u16 & 0x0F);

    RawIns::U16(opcode)
}
