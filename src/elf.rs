use crate::byte_reader::ByteReader;
use crate::section::{read_sections, Section};
use crate::types::{Bits, FromBytes, VariableBits};

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
struct FileIdent {
    bits: Bits,
    endian: Endian,
}

#[derive(Debug)]
pub struct FileHeader {
    e_ident: FileIdent,
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
    e_shstrndx: usize,
}

#[derive(Debug)]
pub struct Elf {
    pub header: FileHeader,
    pub sections: Vec<Section>,
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

    let mut reader = ByteReader::new(&bytes[5..], &bits);

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
    let entry_addr = reader.word();

    // 0x20..0x28 for 64
    // 0x1C..0x20 for 32
    let prog_header_off = reader.word();

    // 0x28..0x30 for 64
    // 0x20..0x24 for 32
    let sec_header_off = reader.word();

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
    let sec_names_idx = reader.read(2, u16::from_bytes) as usize;

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
