use std::ffi;
use std::fmt::Display;
use std::mem::MaybeUninit;
use std::ptr;

#[derive(Debug)]
pub enum VariableBits {
    U64(u64),
    U32(u32),
}

impl VariableBits {
    pub fn usize(&self) -> usize {
        use VariableBits::*;
        match *self {
            U64(u) => u as usize,
            U32(u) => u as usize,
        }
    }
}

impl Display for VariableBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use VariableBits::*;
        match *self {
            U64(u) => write!(f, "{}", u),
            U32(u) => write!(f, "{}", u),
        }
    }
}

impl From<&[u8]> for VariableBits {
    fn from(value: &[u8]) -> Self {
        if value.len() == 4 {
            VariableBits::U32(u32::from_le_bytes(value.try_into().unwrap()))
        } else if value.len() == 8 {
            VariableBits::U64(u64::from_le_bytes(value.try_into().unwrap()))
        } else {
            panic!("variable bits array isnt 4 or 8 byte aligned")
        }
    }
}

#[derive(Debug)]
pub enum Bits {
    B64,
    B32,
}

pub trait FromBytes {
    type T;
    fn from_bytes(bytes: &[u8]) -> Self::T;
}

impl FromBytes for u16 {
    type T = u16;
    fn from_bytes(bytes: &[u8]) -> Self::T {
        if bytes.len() != 2 {
            panic!("unsupported byte array length");
        }
        let mut arr: [u8; 2] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), arr.as_mut_ptr(), 2);
        }
        u16::from_le_bytes(arr)
    }
}

impl FromBytes for u32 {
    type T = u32;
    fn from_bytes(bytes: &[u8]) -> Self::T {
        if bytes.len() != 4 {
            panic!("unsupported byte array length");
        }
        let mut arr: [u8; 4] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe {
            ptr::copy_nonoverlapping(bytes.as_ptr(), arr.as_mut_ptr(), 4);
        }
        u32::from_le_bytes(arr)
    }
}

pub fn get_name(offset: usize, str_tab: &[u8]) -> String {
    let cs = ffi::CStr::from_bytes_until_nul(&str_tab[offset..]).expect("did not contain nul");
    let s = cs.to_str().expect("did not contain valid utf8");
    String::from(s)
}
