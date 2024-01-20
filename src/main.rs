#![allow(dead_code)]
mod byte_reader;
mod elf;
mod section;
mod types;

use std::fs;

use crate::section::SectionType;

fn main() {
    let files = [
        // "samples/hello_world.o",
        // "samples/zero.o",
        // "samples/clo.o",
        "samples/more.o",
        // "samples/hello.o", // smaller rust ELF
        // "samples/this.o", // this ELF is this programs ELF, though it may be outdated. it is
        // just to check if we can read large elfs
    ];

    for file in files {
        println!("{file}:\n");
        let elf_bytes: Vec<u8> = fs::read(file).unwrap();

        let elf = elf::read_elf(elf_bytes);
        println!("{elf}\n");
        for section in &elf.sections {
            if section.header.sh_type != SectionType::Null {
                println!("  {section}");
            }
        }
        println!("");

        // elf::read_symtab(&elf);
        elf::read_relas(&elf);
        println!("");
    }
}
