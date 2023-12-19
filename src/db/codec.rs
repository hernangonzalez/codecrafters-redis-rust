// #[allow(dead_code)]
// #[repr(u8)]
// pub enum Kind {
//     String = 0,
//     List = 1,
//     Set = 2,
//     SortedSet = 3,
//     Hash = 4,
//     ZipMap = 9,
//     ZipList = 10,
//     IntSet = 11,
//     SortedSetZipList = 12,
//     HashMapZipList = 13,
//     QuickList = 14,
// }

use bytes::BytesMut;
use std::io::Read;

pub mod string {
    use super::*;

    const LENGTH_BITMASK: u8 = 0b11000000u8;
    const LENGTH_READY: u8 = 0u8;
    const LENGTH_READ_MORE: u8 = 0b01000000u8;
    const LENGTH_NEXT_4: u8 = 0b10000000u8;
    const LENGTH_FORMAT: u8 = 0b11000000u8;

    #[allow(dead_code)]
    #[derive(Debug)]
    enum StringMask {
        String(usize),
        Int(u8),
        Compressed,
    }

    pub fn read(reader: &mut impl Read) -> anyhow::Result<String> {
        let kind = encoded_string_mask(reader)?;
        let str = match kind {
            StringMask::String(len) => {
                let mut buf = BytesMut::zeroed(len);
                reader.read_exact(&mut buf)?;
                std::str::from_utf8(&buf)?.to_string()
            }
            StringMask::Int(len) => {
                let val: u32 = match len {
                    0 => {
                        let mut buf: [u8; 1] = [0; 1];
                        reader.read_exact(&mut buf)?;
                        u8::from_ne_bytes(buf) as u32
                    }
                    1 => {
                        let mut buf: [u8; 2] = [0; 2];
                        reader.read_exact(&mut buf)?;
                        u16::from_ne_bytes(buf) as u32
                    }
                    2 => {
                        let mut buf: [u8; 4] = [0; 4];
                        reader.read_exact(&mut buf)?;
                        u32::from_ne_bytes(buf) as u32
                    }
                    _ => panic!("Unsupported"),
                };
                val.to_string()
            }
            StringMask::Compressed => panic!("Not implemented"),
        };
        Ok(str)
    }

    fn encoded_string_mask(reader: &mut impl Read) -> anyhow::Result<StringMask> {
        let mut buf = [0; 1];
        reader.read_exact(&mut buf)?;
        read_string_mask(buf[0], reader)
    }

    fn read_string_mask(mask: u8, reader: &mut impl Read) -> anyhow::Result<StringMask> {
        let mask = match mask & LENGTH_BITMASK {
            LENGTH_READY => StringMask::String(mask as usize),
            LENGTH_READ_MORE => {
                let mut buf2 = [0; 1];
                reader.read_exact(&mut buf2)?;
                let val: usize = ((mask & !LENGTH_BITMASK) as usize) << 8;
                let val = val | (buf2[0] as usize);
                StringMask::String(val)
            }
            LENGTH_NEXT_4 => {
                let mut buf2 = [0; 4];
                reader.read_exact(&mut buf2)?;
                let val = u32::from_ne_bytes(buf2) as usize;
                StringMask::String(val)
            }
            LENGTH_FORMAT => {
                let len = mask & !LENGTH_BITMASK;
                StringMask::Int(len)
            }
            _ => panic!("Unreachable"),
        };
        Ok(mask)
    }
}
