// handy byte conversion functions
//
pub fn as_u64_le(array: &[u8]) -> u64 {
    ((array[0] as u64) << 0)
        + ((array[1] as u64) << 8)
        + ((array[2] as u64) << 16)
        + ((array[3] as u64) << 24)
        + ((array[4] as u64) << 32)
        + ((array[5] as u64) << 40)
        + ((array[6] as u64) << 48)
        + ((array[7] as u64) << 56)
}

pub fn as_u32_le(array: &[u8]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}

pub fn as_u16_le(array: &[u8]) -> u16 {
    ((array[0] as u16) << 0) + ((array[1] as u16) << 8)
}


