use crate::types::{Bits, VariableBits};
use Bits::*;

pub struct ByteReader<'data> {
    data: &'data [u8],
    offset: usize,
    bits: &'data Bits,
}

impl<'data> ByteReader<'data> {
    pub fn new(d: &'data [u8], bits: &'data Bits) -> Self {
        Self {
            data: d,
            offset: 0,
            bits,
        }
    }

    pub fn word(&'data mut self) -> VariableBits {
        let start = self.offset;
        match self.bits {
            B32 => {
                self.offset += 4;
            }
            B64 => {
                self.offset += 8;
            }
        }
        let end = self.offset;
        let data = match self.bits {
            B32 => &self.data[start..end],
            B64 => &self.data[start..end],
        };

        VariableBits::from(data)
    }

    pub fn read<T, F>(&'data mut self, bytes: usize, convert: F) -> T
    where
        F: Fn(&'data [u8]) -> T,
    {
        let data = &self.data[self.offset..self.offset + bytes];
        self.offset += bytes;
        convert(data)
    }
}
