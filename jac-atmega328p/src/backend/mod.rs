use std::fmt::Debug;

pub mod encode;
pub mod ins;

pub enum RawIns {
    U16(u16),
    U32(u32),
}

impl Debug for RawIns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawIns::U16(v) => write!(f, "U16({:x})", v),
            RawIns::U32(v) => write!(f, "U32({:x})", v),
        }
    }
}


