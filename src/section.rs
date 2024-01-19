struct SectionHeader {}

fn make_section(bytes: &[u8], bits: &crate::Bits) -> SectionHeader {
    let sh_name = u32::from_le_bytes(bytes[..4].try_into().unwrap());
    let sh_type = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    let mut offset = 8;

    SectionHeader {}
}

pub fn read_sections(bytes: &[u8], entries: u16, size: u16, _nameidx: u16, bits: &crate::Bits) {
    let mut offset = 0;
    let mut sec_headers = vec![];
    for _ in 0..entries {
        let header = &bytes[offset..offset + size as usize];
        sec_headers.push(header);
        offset += size as usize;
    }

    for bytes in sec_headers {
        make_section(bytes, bits);
    }
}
