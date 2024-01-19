use crate::byte_reader::ByteReader;
use crate::types::Bits;
use crate::types::FromBytes;
use crate::types::VariableBits;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ffi;

#[derive(Debug, FromPrimitive)]
pub enum SectionType {
    Null,
    ProgBits,
    SymTab,
    StrTab,
    Rela,
    Hash,
    Dynamic,
    Note,
    Nobits,
    Rel,
    Shlib,
    Dynsym,
    InitArray,
    FiniArray,
    PreinitArray,
    Group,
    SymtabShndx,
    Num,
}

#[derive(Debug)]
pub struct SectionHeader {
    pub sh_name: u32,
    pub sh_type: SectionType,
    pub sh_flags: VariableBits,
    pub sh_addr: VariableBits,
    pub sh_offset: VariableBits,
    pub sh_size: VariableBits,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: VariableBits,
    pub sh_entsize: VariableBits,
}

impl SectionHeader {
    pub fn name(&self, str_tab: &[u8]) -> String {
        let offset = self.sh_name as usize;
        let cs = ffi::CStr::from_bytes_until_nul(&str_tab[offset..]).expect("did not contain nul");
        let s = cs.to_str().expect("did not contain valid utf8");
        String::from(s)
    }
}

#[derive(Debug)]
pub struct Section {
    pub header: SectionHeader,
    pub name: String,
    pub data: Vec<u8>,
}

impl Section {
    pub fn dump(&self) {
        println!(
            "{} {:?} {} {} {:?}",
            self.name, self.header.sh_type, self.header.sh_offset, self.header.sh_size, self.data
        )
    }
}

fn make_section_header(bytes: &[u8], bits: &Bits) -> SectionHeader {
    let mut reader = ByteReader::new(bytes, bits);
    let sh_name = reader.read(4, u32::from_bytes);
    let sh_type =
        SectionType::from_u32(reader.read(4, u32::from_bytes)).expect("unknown section type");
    let sh_flags = reader.word();
    let sh_addr = reader.word();
    let sh_offset = reader.word();
    let sh_size = reader.word();
    let sh_link = reader.read(4, u32::from_bytes);
    let sh_info = reader.read(4, u32::from_bytes);
    let sh_addralign = reader.word();
    let sh_entsize = reader.word();

    SectionHeader {
        sh_name,
        sh_type,
        sh_flags,
        sh_addr,
        sh_offset,
        sh_size,
        sh_link,
        sh_info,
        sh_addralign,
        sh_entsize,
    }
}

pub fn read_section_headers(
    bytes: &[u8],
    entries: u16,
    size: u16,
    bits: &Bits,
) -> Vec<SectionHeader> {
    let mut offset = 0;
    let mut sec_headers = vec![];
    for _ in 0..entries {
        let header = &bytes[offset..offset + size as usize];
        sec_headers.push(header);
        offset += size as usize;
    }

    sec_headers
        .iter()
        .map(|h| make_section_header(h, bits))
        .collect()
}

fn make_section(header: SectionHeader, bytes: &[u8], shstrtab: &[u8]) -> Section {
    let data = Vec::from(
        &bytes[header.sh_offset.usize()..header.sh_offset.usize() + header.sh_size.usize()],
    );

    let name = header.name(shstrtab);

    Section { header, data, name }
}

pub fn read_sections(
    bytes: &[u8],
    header_offset: usize,
    entries: u16,
    size: u16,
    nameidx: usize,
    bits: &Bits,
) -> Vec<Section> {
    let headers = read_section_headers(&bytes[header_offset..], entries, size, bits);

    let shstrtab_header = &headers[nameidx];
    let shstrtab = &bytes[shstrtab_header.sh_offset.usize()
        ..shstrtab_header.sh_offset.usize() + shstrtab_header.sh_size.usize()];

    headers
        .into_iter()
        .map(|h| make_section(h, bytes, shstrtab))
        .collect()
}
