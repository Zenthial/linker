#![allow(dead_code)]
mod byte_reader;
mod elf;
mod section;
mod types;

use std::fs;

// use crate::section::SectionType;

fn main() {
    // println!("{:?}", std::env::args());
    let files = [
        "samples/hello_world.o",
        "samples/zero.o",
        "samples/clo.o",
        "samples/more.o",
        "samples/fib.o",
    ];

    for file in files {
        println!("{file}:\n");
        let elf_bytes: Vec<u8> = fs::read(file).unwrap();

        let elf = elf::read_elf(elf_bytes);
        // println!("{elf}\n");
        // for section in &elf.sections {
        //     if section.header.sh_type != SectionType::Null {
        //         println!("  {section}");
        //     }
        // }
        // println!("");

        let symbols = elf::read_symtab(&elf);
        let needs_definition = elf::collect_unknown_symbols(symbols);
        for sym in needs_definition {
            println!("  {sym:?}");
        }
        elf::read_relas(&elf);
        println!("");
    }
}
