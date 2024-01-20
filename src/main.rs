// reference: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
// elf32 abi specification: https://www.sco.com/developers/devspecs/gabi41.pdf
// elf64 abi specification: https://uclibc.org/docs/elf-64-gen.pdf

#![allow(dead_code)]
mod byte_reader;
mod elf;
mod section;
mod types;

use std::fs;

fn main() {
    let files = [
        "samples/hello_world.o",
        "samples/zero.o",
        "samples/clo.o",
        "samples/more.o",
    ];

    for file in files {
        println!("{file}:");
        let elf_bytes: Vec<u8> = fs::read(file).unwrap();

        let elf = elf::read_elf(elf_bytes);
        // println!("{elf:?}");
        // for section in &elf.sections {
        //     section.dump();
        // }

        // elf::read_symtab(&elf);
        elf::read_rela(&elf);
        println!("");
    }
}
