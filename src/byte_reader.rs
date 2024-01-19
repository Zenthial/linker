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

    /// word might not be the best descriptor
    /// reads 4 or 8 bytes depending on if the byte reader is in
    /// 32 or 64 bit mode
    pub fn word(&mut self) -> VariableBits {
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

    /// read N amount of bytes and apply a conversion function 
    /// to transform a byte array to a more useable type
    pub fn read<T, F>(&mut self, bytes: usize, convert: F) -> T
    where
        F: Fn(&'data [u8]) -> T,
    {
        let data = &self.data[self.offset..self.offset + bytes];
        self.offset += bytes;
        convert(data)
    }

    /// read N amount of bytes, and return the byte slice
    pub fn read_raw(&mut self, bytes: usize) -> &'data [u8] {

        let data = &self.data[self.offset..self.offset + bytes];
        self.offset += bytes;
        data
    }

    pub fn byte(&mut self) -> u8 {
        let data = &self.data[self.offset];
        self.offset += 1;
        *data
    }
}
