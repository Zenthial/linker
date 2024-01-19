// reference: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
#![allow(dead_code)]
mod bytes;

use std::fs;
use crate::bytes::*;


#[derive(Debug)]
enum Bits {
    B64,
    B32,
}

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
    entry_addr: Vec<u8>,
    prog_header_off: u64, // program header offset
    sec_header_off: u64,  // section header offset
    flags: u32,           // platform specific, may not even need?
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

    let endian = match bytes[0x5] {
        0x1 => Endian::Little,
        0x2 => Endian::Big,
        _ => panic!("bits unrecoginzed"),
    };

    if bytes[0x7] != 0x3 && bytes[0x7] != 0x0 {
        panic!("only support linux");
    }

    let ty = match bytes[0x10] {
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
        Bits::B64 => Vec::from(&bytes[0x18..0x20]),
        Bits::B32 => Vec::from(&bytes[0x18..0x1C]),
    };

    let prog_header_off = match bits {
        Bits::B64 => as_u64_le(&bytes[0x20..0x28]),
        Bits::B32 => as_u64_le(&bytes[0x1C..0x20]),
    };

    let sec_header_off = match bits {
        Bits::B64 => as_u64_le(&bytes[0x28..0x30]),
        Bits::B32 => as_u64_le(&bytes[0x20..0x24]),
    };

    let mut offset = match bits {
        Bits::B64 => 0x30,
        Bits::B32 => 0x24,
    };

    let flags = as_u32_le(&bytes[offset..offset + 4]);
    offset += 4;

    let file_header_size = as_u16_le(&bytes[offset..offset + 2]);
    offset += 2;
    let prog_header_size = as_u16_le(&bytes[offset..offset + 2]);
    offset += 2;
    let prog_entries = as_u16_le(&bytes[offset..offset + 2]);
    offset += 2;
    let sec_header_size = as_u16_le(&bytes[offset..offset + 2]);
    offset += 2;
    let sec_entries = as_u16_le(&bytes[offset..offset + 2]);
    offset += 2;
    let sec_names_idx = as_u16_le(&bytes[offset..offset + 2]);

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
    let elf_bytes: Vec<u8> = fs::read("samples/zero.o").unwrap();

    println!("{:?}", read_elf(elf_bytes));

    let elf_bytes: Vec<u8> = fs::read("samples/hello_world.o").unwrap();

    println!("{:?}", read_elf(elf_bytes));
}
