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

use anyhow::Result;
use bytes::BytesMut;
use std::io::Read;

pub mod length {
    use super::*;

    const LENGTH_BITMASK: u8 = 0b11000000u8;
    const LENGTH_READY: u8 = 0u8;
    const LENGTH_READ_MORE: u8 = 0b01000000u8;
    const LENGTH_NEXT_4: u8 = 0b10000000u8;
    const LENGTH_FORMAT: u8 = 0b11000000u8;

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum Length {
        Read(usize),
        Value(u8),
        Compressed,
    }

    impl Into<usize> for Length {
        fn into(self) -> usize {
            match self {
                Self::Read(s) => s,
                Self::Value(v) => v as usize,
                Length::Compressed => panic!("to be implemented"),
            }
        }
    }

    pub fn read(reader: &mut impl Read) -> Result<Length> {
        let mut buf = [0; 1];
        reader.read_exact(&mut buf)?;
        read_mask(buf[0], reader)
    }

    fn read_mask(mask: u8, reader: &mut impl Read) -> Result<Length> {
        let mask = match mask & LENGTH_BITMASK {
            LENGTH_READY => Length::Read(mask as usize),
            LENGTH_READ_MORE => {
                let mut buf2 = [0; 1];
                reader.read_exact(&mut buf2)?;
                let val: usize = ((mask & !LENGTH_BITMASK) as usize) << 8;
                let val = val | (buf2[0] as usize);
                Length::Read(val)
            }
            LENGTH_NEXT_4 => {
                let mut buf2 = [0; 4];
                reader.read_exact(&mut buf2)?;
                let val = u32::from_ne_bytes(buf2) as usize;
                Length::Read(val)
            }
            LENGTH_FORMAT => {
                let len = mask & !LENGTH_BITMASK;
                Length::Value(len)
            }
            _ => panic!("Unreachable"),
        };
        Ok(mask)
    }
}

pub mod string {
    use super::length::Length;
    use super::*;

    pub fn read(reader: &mut impl Read) -> Result<String> {
        let kind = length::read(reader)?;
        let str = match kind {
            Length::Read(len) => {
                let mut buf = BytesMut::zeroed(len);
                reader.read_exact(&mut buf)?;
                std::str::from_utf8(&buf)?.to_string()
            }
            Length::Value(len) => {
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
            Length::Compressed => panic!("Not implemented"),
        };
        Ok(str)
    }
}
