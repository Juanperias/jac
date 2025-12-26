// the opcodes need to be encoded in le bytes
mod backend;
mod frontend;

use bit_field::BitField;
use jihex::{Code, Record};

use crate::backend::encode::encode;

fn main() {
    let a = "
        ldi r16, 255
    ";
    
    let program = frontend::parser::program(a).unwrap();
    
    for i in &program.0 {
        println!("{:?}", encode(i.clone()));
    }

    let imm = 0xff_u8;
    let opcode: u16 =
        (0b1110 << 12) | ((imm as u16 & 0xF0) << 4) | ((0 as u8) << 4) as u16 | (imm as u16 & 0x0F);
    println!("{:04X}", opcode);

    println!("{:?}", program);

    let mut s = String::new();

    let code = Code(vec![
        Record::ExtendedSegmentAddress(0x0),
        Record::Data(&[
            0x54, 0x9A, 0x55, 0x9A, 0x0E, 0x94, 0x13, 0x00, 0x91, 0xE0, 0x84, 0xEF, 0x0E, 0x94,
            0x17, 0x00,
        ]),
        Record::Data(&[
            0x0E, 0x94, 0x13, 0x00, 0x0E, 0x94, 0x15, 0x00, 0x91, 0xE0, 0x84, 0xEF, 0x0E, 0x94,
            0x17, 0x00,
        ]),
        Record::Data(&[
            0x0E, 0x94, 0x15, 0x00, 0xEF, 0xCF, 0x4C, 0x9A, 0x08, 0x95, 0x4D, 0x9A, 0x08, 0x95,
            0xFF, 0xE0,
        ]),
        Record::Data(&[
            0xE0, 0xEA, 0x31, 0x97, 0xF1, 0xF7, 0x01, 0x97, 0xD1, 0xF7, 0x08, 0x95,
        ]),
        Record::Eof,
    ]);

    code.write(&mut s);

    println!("{s}");
}
