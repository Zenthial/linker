use crate::byte_reader::ByteReader;
use crate::types::Bits;
use crate::types::VariableBits;

fn convert_to_integer<T>(bytes: &[u8]) -> T
where
    T: From<u16> + From<u32> + TryInto<u16> + TryInto<u32>,
{
    match bytes.len() {
        2 => {
            // Convert the byte array to T (either u16 or other types)
            let value = u16::from_le_bytes(bytes.try_into().unwrap());
            value.into()
        }
        4 => {
            // Convert the byte array to T (either u32 or other types)
            let value = u32::from_le_bytes(bytes.try_into().unwrap());
            value.into()
        }
        _ => panic!("Unsupported byte array length"),
    }
}

struct SectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: VariableBits,
}

fn make_section(bytes: &[u8], bits: &Bits) -> SectionHeader {
    let mut reader = ByteReader::new(bytes, bits);
    let sh_name = reader.read(4, convert_to_integer::<u32>);
    let sh_type = reader.read(4, convert_to_integer::<u32>);
    let sh_flags = reader.word();

    SectionHeader {
        sh_name,
        sh_type,
        sh_flags,
    }
}

pub fn read_sections(bytes: &[u8], entries: u16, size: u16, _nameidx: u16, bits: &Bits) {
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
