use crate::{backend::ins::ldi, frontend::parser::Item, backend::RawIns};

pub fn encode(item: Item) -> RawIns { 
    match item {
        Item::Ldi(reg, imm) => ldi(reg as u8, imm),
    }
}

