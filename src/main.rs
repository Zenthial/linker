// reference: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
#![allow(dead_code)]
mod byte_reader;
mod section;
mod types;

use crate::byte_reader::ByteReader;
use crate::section::read_sections;
use crate::types::{Bits, FromBytes, VariableBits};
use std::fs;

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
struct Elf {
    bits: Bits,
    endian: Endian,
    ty: ElfType,
    inst_set: InstructionSet,
    entry_addr: VariableBits,
    prog_header_off: VariableBits, // program header offset
    sec_header_off: VariableBits,  // section header offset
    flags: u32,                    // platform specific, may not even need?
    file_header_size: u16,
    prog_header_size: u16,
    prog_entries: u16,
    sec_header_size: u16,
    sec_entries: u16,
    sec_names_idx: u16,
}

fn read_elf(bytes: Vec<u8>) -> Elf {
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

    let inst_set = match bytes[0x12] {
        0x3E => InstructionSet::AmdX86_64,
        _ => panic!("unknown inst set {}", bytes[0x12]),
    };

    let entry_addr = match bits {
        Bits::B64 => VariableBits::from(&bytes[0x18..0x20]),
        Bits::B32 => VariableBits::from(&bytes[0x18..0x1C]),
    };

    let prog_header_off = match bits {
        Bits::B64 => VariableBits::from(&bytes[0x20..0x28]),
        Bits::B32 => VariableBits::from(&bytes[0x1C..0x20]),
    };

    let sec_header_off = match bits {
        Bits::B64 => VariableBits::from(&bytes[0x28..0x30]),
        Bits::B32 => VariableBits::from(&bytes[0x20..0x24]),
    };

    let mut offset = match bits {
        Bits::B64 => 0x30,
        Bits::B32 => 0x24,
    };

    let flags = u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
    offset += 4;

    let file_header_size = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
    offset += 2;
    let prog_header_size = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
    offset += 2;
    let prog_entries = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
    offset += 2;
    let sec_header_size = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
    offset += 2;
    let sec_entries = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());
    offset += 2;
    let sec_names_idx = u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap());

    read_sections(
        &bytes[sec_header_off.usize()..],
        sec_entries,
        sec_header_size,
        sec_names_idx,
        &bits,
    );

    Elf {
        bits,
        endian,
        ty,
        inst_set,
        entry_addr,
        prog_header_off,
        sec_header_off,
        flags,
        file_header_size,
        prog_header_size,
        prog_entries,
        sec_entries,
        sec_names_idx,
        sec_header_size,
    }
}

fn main() {
    // let elf_bytes: Vec<u8> = fs::read("samples/zero.o").unwrap();
    //
    // println!("{:?}", read_elf(elf_bytes));

    let elf_bytes: Vec<u8> = fs::read("samples/hello_world.o").unwrap();

    println!("{:?}", read_elf(elf_bytes));
}
