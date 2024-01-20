use std::fmt::Display;

use crate::byte_reader::ByteReader;
use crate::section::{read_sections, Section, SectionType};
use crate::types::{get_name, Bits, FromBytes, VariableBits};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug)]
enum Endian {
    Big,
    Little,
}

#[derive(Debug)]
enum ElfType {
    None, // 0x0
    Rel,
    Exec,
    Dyn,
    Core, // 0x4
}

#[derive(Debug)]
enum InstructionSet {
    AmdX86_64,
}

#[derive(Debug)]
pub struct FileIdent {
    pub bits: Bits,
    endian: Endian,
}

#[derive(Debug)]
pub struct FileHeader {
    pub e_ident: FileIdent,
    e_type: ElfType,
    e_machine: InstructionSet,
    e_entry: VariableBits,
    e_phoff: VariableBits, // program header offset
    e_shoff: VariableBits, // section header offset
    e_flags: u32,          // platform specific, may not even need?
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    pub e_shstrndx: usize,
}

#[derive(Debug)]
pub struct Elf {
    pub header: FileHeader,
    pub sections: Vec<Section>,
}

impl Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec_string = String::new();
        for s in &self.sections {
            vec_string += &format!("[{s}] ");
        }

        write!(f, "{:?} {}", self.header, vec_string)
    }
}

#[derive(Debug, FromPrimitive)]
enum SymbolBinding {
    Local,
    Global,
    Weak,
    Loos = 10,
    Hios = 12,
    Loproc,
    Hiproc = 15,
}

#[derive(Debug, FromPrimitive, PartialEq, Eq)]
enum SymbolType {
    NoType,
    Object,
    Func,
    Section,
    File,
    Loos = 10,
    Hios = 12,
    Loproc,
    Hiproc = 15,
}

#[derive(Debug)]
struct SymbolInfo {
    binding: SymbolBinding,
    ty: SymbolType,
}

#[derive(Debug)]
pub struct SymbolTableEntry {
    st_name: String,     // u32 offset, but we just ignore that and fill it in
    st_info: SymbolInfo, // expand the one byte into a useable struct
    // there is another byte of padding here that isn't used by the entry, so we don't include it
    st_shndx: u16,
    st_value: VariableBits,
    st_size: VariableBits,
}

fn read_entry(
    entry_bytes: &[u8],
    bits: &Bits,
    str_tab: &[u8],
    sections: &Vec<Section>,
) -> SymbolTableEntry {
    let mut reader = ByteReader::new(entry_bytes, bits);
    let st_name = reader.read(4, u32::from_bytes); // 0x4
    let info: u8 = reader.byte(); // 0x4

    let binding_info = info >> 4;
    let type_info = info & 0b00001111;
    let binding = SymbolBinding::from_u8(binding_info)
        .expect(&format!("no binding info for {:04b}", binding_info));
    let ty = SymbolType::from_u8(type_info).expect(&format!("no type info for {:04b}", type_info));

    let _other = reader.byte(); // padding
    let st_shndx = reader.read(2, u16::from_bytes);
    let st_value = reader.addr();
    let st_size = reader.addr(); // this is technically an 'xword', but is also 4 or 8 bytes depending on arch, so we just use addr

    let mut name = get_name(st_name as usize, str_tab);
    if ty == SymbolType::Section {
        name = sections[(st_shndx ) as usize].name.clone();
    }

    SymbolTableEntry {
        st_name: name,
        st_info: SymbolInfo { binding, ty },
        st_shndx,
        st_value,
        st_size,
    }
}

pub fn read_symtab(elf: &Elf) -> Vec<SymbolTableEntry> {
    let symtab = elf.sections.iter().find(|s| s.name == ".symtab");
    let symtab = match symtab {
        Some(st) => st,
        None => panic!("no .symtab"),
    };
    let bits = &elf.header.e_ident.bits;
    let str_tab = elf.sections.iter().find(|s| s.name == ".strtab");
    let str_tab = match str_tab {
        Some(st) => &st.data,
        None => panic!("no .strtab"),
    };

    let mut entries = vec![];
    let symbols = symtab.header.entries();
    let mut offset = 0; // we index one entry in, because the first
                                                       // entry is always all 0s
    for _ in 0..symbols {
        let bytes = &symtab.data[offset..offset + symtab.header.sh_entsize.usize()];
        entries.push(read_entry(bytes, bits, str_tab, &elf.sections));
        offset += symtab.header.sh_entsize.usize();
    }

    entries
}

//Advanced Micro Devices X86-64 relocation types
#[derive(Debug, FromPrimitive)]
#[allow(non_camel_case_types)]
enum RelaType {
    R_X86_64_64 = 1,
    R_X86_64_PC32 = 2,
    R_X86_64_PLT32 = 4,
    R_X86_64_GOTPCREL = 9,
    R_X86_64_32 = 10,
}

#[derive(Debug)]
pub struct Rela {
    r_name: String, // field we resolve
    r_offset: u64,
    r_ty: RelaType, // info gets removed and replaced with ty
    r_addend: i64,
}

impl Display for Rela {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:012x} {:?} {} {}", self.r_offset, self.r_ty, self.r_name, self.r_addend)
    }
}

// this only supports 64 bit for now
fn read_rela(mut reader: ByteReader, symbs: &Vec<SymbolTableEntry>) -> Rela {
    let r_offset = reader.read(8, u64::from_bytes);
    let r_info = reader.read(8, u64::from_bytes);
    let r_addend = reader.read(8, i64::from_bytes);

    let r_sym = u32::from_u64(r_info >> 32).expect("bitshift error?") ;
    let r_ty = u32::from_u64(r_info & 0xffffffff).expect("bit mask error?");
    let r_ty = match RelaType::from_u32(r_ty) {
        Some(ty) => ty,
        None => {
            panic!("{r_ty}");
        }
    };

    let r_name = symbs[r_sym as usize].st_name.clone();

    Rela {
        r_name,
        r_offset,
        r_ty,
        r_addend,
    }
}

pub fn read_relas(elf: &Elf) -> Vec<(String, Vec<Rela>)> {
    let bits = &elf.header.e_ident.bits;

    let relas: Vec<&Section> = elf
        .sections
        .iter()
        .filter(|s| s.header.sh_type == SectionType::Rela)
        .collect();

    let symbs = read_symtab(elf);

    let mut rs = vec![];
    for rela in relas {
        let mut relocs = vec![];
        let entries = rela.header.entries();
        println!("{}", rela.name);
        let mut offset = 0; // we index one entry in, because the first
        for _ in 0..entries {
            let reader = ByteReader::new(
                &rela.data[offset..offset + rela.header.sh_entsize.usize()],
                bits,
            );
            let r = read_rela(reader, &symbs);
            println!("  {r}");
            relocs.push(r);
            offset += rela.header.sh_entsize.usize();
        }

        rs.push((rela.name.clone(), relocs));
    }

    rs
}

pub fn read_elf(bytes: Vec<u8>) -> Elf {
    // these four bytes are the ELF magic number
    if &bytes[..0x4] != &[0x7F, 0x45, 0x4c, 0x46] {
        panic!("not an elf file");
    }

    let bits = match bytes[0x4] {
        0x1 => Bits::B32,
        0x2 => Bits::B64,
        _ => panic!("bits unrecoginzed"),
    };

    let mut reader = ByteReader::new(&bytes[0x5..], &bits);

    let endian = match reader.byte() {
        0x1 => Endian::Little,
        0x2 => Endian::Big,
        _ => panic!("bits unrecoginzed"),
    };

    // version is always 1
    // 0x6
    if reader.byte() != 0x1 {
        panic!("somehow elf version isnt 1")
    }

    // 0x7
    let platform = reader.byte();
    if platform != 0x3 && platform != 0x0 {
        panic!("only support linux");
    }

    let _abi_version_and_padding = reader.read_raw(8);
    // 0x10, 0x11
    let ty = match reader.read(2, u16::from_bytes) {
        0x0 => ElfType::None,
        0x1 => ElfType::Rel,
        0x2 => ElfType::Exec,
        0x3 => ElfType::Dyn,
        0x4 => ElfType::Core,
        _ => panic!("bits unrecoginzed"),
    };

    // 0x12, 0x13
    let inst_set = match reader.read(2, u16::from_bytes) {
        0x3E => InstructionSet::AmdX86_64,
        _ => panic!("unknown inst set {}", bytes[0x12]),
    };

    // 0x14 -> 0x17
    let _version = reader.read(4, u32::from_bytes);

    // 0x18..0x20 for 64
    // 0x18..0x1C for 32
    let entry_addr = reader.addr();

    // 0x20..0x28 for 64
    // 0x1C..0x20 for 32
    let prog_header_off = reader.addr();

    // 0x28..0x30 for 64
    // 0x20..0x24 for 32
    let sec_header_off = reader.addr();

    // 0x24 or 0x30
    let flags = reader.read(4, u32::from_bytes);
    // 0x28 or 0x34
    let file_header_size = reader.read(2, u16::from_bytes);
    // 0x2A or 0x36
    let prog_header_size = reader.read(2, u16::from_bytes);
    // 0x2C or 0x38
    let prog_entries = reader.read(2, u16::from_bytes);
    // 0x2E or 0x3A
    let sec_header_size = reader.read(2, u16::from_bytes);
    // 0x30 or 0x3C
    let sec_entries = reader.read(2, u16::from_bytes);
    // 0x32 or 0x3E
    let sec_names_idx = reader.read(2, u16::from_bytes) as usize ; 

    let sections = read_sections(
        &bytes,
        sec_header_off.usize(),
        sec_entries,
        sec_header_size,
        sec_names_idx,
        &bits,
    );

    let header = FileHeader {
        e_ident: FileIdent { bits, endian },
        e_type: ty,
        e_machine: inst_set,
        e_entry: entry_addr,
        e_phoff: prog_header_off,
        e_shoff: sec_header_off,
        e_flags: flags,
        e_ehsize: file_header_size,
        e_phentsize: prog_header_size,
        e_phnum: prog_entries,
        e_shentsize: sec_header_size,
        e_shnum: sec_entries,
        e_shstrndx: sec_names_idx,
    };

    Elf { header, sections }
}
