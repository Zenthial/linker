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
