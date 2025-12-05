use std::fmt::Write;
use thiserror::Error;

#[derive(Debug)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

trait Encode<T> {
    fn encode(&self, n: T) -> Vec<u8>;
}

macro_rules! num_impl {
    ($($n: ident)*) => {
        $(
            impl Encode<$n> for Endianness {
                fn encode(&self, n: $n) -> Vec<u8> {
                    match self {
                        Endianness::LittleEndian => n.to_le_bytes().to_vec(),
                        Endianness::BigEndian => n.to_le_bytes().to_vec(),
                    }
                }
            }
        )*
    }
}

num_impl! { u16 u32 u64 }

pub struct Code(pub Vec<Record>);

impl Code {
    pub fn write<T: Write>(&self, dst: &mut T) {
        let mut address = 0;
        let data = self.process(&mut address);

        for record in data {
            dst.write_char(':').unwrap();
            dst.write_str(&format!("{:02X}", record.size)).unwrap();
            dst.write_str(&format!("{:04X}", record.address)).unwrap();
            dst.write_str(&format!("{:02X}", record.record_type))
                .unwrap();

            for c in record.data {
                dst.write_str(&format!("{:02X}", c)).unwrap();
            }

            dst.write_str(&format!("{:02X}", record.checksum)).unwrap();

            dst.write_char('\n').unwrap();
        }
    }

    fn process(&self, address: &mut u16) -> Vec<RecordData> {
        self.0.iter().map(|v| RecordData::new(v, address)).collect()
    }
    // who in hell gonna use write_async(?
}

pub enum Record {
    ExtendedSegmentAddress(u16),
    Data(&'static [u8]),
    Eof,
}

impl Record {}

#[derive(Debug)]
struct RecordData {
    size: u8,
    address: u16,
    record_type: u8,
    data: &'static [u8],
    checksum: u8,
}

impl RecordData {
    // takes a high level and return a low level record
    fn new(record: &Record, address: &mut u16) -> Self {
        match record {
            Record::ExtendedSegmentAddress(a) => RecordData {
                size: 0x02,
                address: 0,
                record_type: 0x2,
                data: &[0x0, 0x0],
                checksum: 0xFC,
            },
            Record::Data(d) => {
                let size: u8 = d.len().try_into().unwrap();
                *address += size as u16;
                let a = RecordData {
                    size,
                    address: *address,
                    data: d,
                    record_type: 0,
                    checksum: 0xfc,
                };

                RecordData {
                    checksum: checksum(&a),
                    ..a
                }
            }
            Record::Eof => {
                // I'm hardcoding the checksum because is always static values and is always the same checksum
                let a = RecordData {
                    size: 0,
                    address: 0,
                    data: &[],
                    record_type: 1,
                    checksum: 0,
                };
                RecordData {
                    checksum: 0xff,
                    ..a
                }
            }
        }
    }
}

fn checksum(data: &RecordData) -> u8 {
    let sum = (data.size as u32)
        + (data.address as u32)
        + (data.data.iter().map(|i| *i as u32).sum::<u32>())
        + (data.record_type as u32);

    ((0x100 - (sum & 0xff)) & 0xFF) as u8
}

#[derive(Error, Debug)]
pub enum IHexError {}
