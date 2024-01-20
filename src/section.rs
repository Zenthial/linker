use crate::byte_reader::ByteReader;
use crate::types::get_name;
use crate::types::Bits;
use crate::types::FromBytes;
use crate::types::VariableBits;

use std::fmt::Display;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, FromPrimitive, PartialEq, Eq)]
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

    LOOS = 1879002115,         // another llvm thing
    X86_64Unwind = 1879048193, // some thing that llvm uses
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
    pub fn entries(&self) -> usize {
        self.sh_size.usize() / self.sh_entsize.usize()
    }
}

#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub header: SectionHeader,
    pub data: Vec<u8>,
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "section: {}, type: {:?}, offset: {}, size: {}, entry_size: {}",
            self.name,
            self.header.sh_type,
            self.header.sh_offset,
            self.header.sh_size,
            self.header.sh_entsize,
        )
    }
}

fn make_section_header(bytes: &[u8], bits: &Bits) -> SectionHeader {
    let mut reader = ByteReader::new(bytes, bits);
    let sh_name = reader.read(4, u32::from_bytes);
    let sh_type_value = reader.read(4, u32::from_bytes);
    let sh_type = SectionType::from_u32(sh_type_value)
        .expect(&format!("unknown section type: {}", sh_type_value));
    let sh_flags = reader.addr();
    let sh_addr = reader.addr();
    let sh_offset = reader.addr();
    let sh_size = reader.addr();
    let sh_link = reader.read(4, u32::from_bytes);
    let sh_info = reader.read(4, u32::from_bytes);
    let sh_addralign = reader.addr();
    let sh_entsize = reader.addr();

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

fn make_section(header: SectionHeader, bytes: &[u8], str_tab: &[u8]) -> Section {
    let data = Vec::from(
        &bytes[header.sh_offset.usize()..header.sh_offset.usize() + header.sh_size.usize()],
    );

    let offset = header.sh_name as usize;
    let name = get_name(offset, str_tab);

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
