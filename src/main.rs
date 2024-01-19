// reference: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
#![allow(dead_code)]
mod byte_reader;
mod section;
mod types;
mod elf;

use std::fs;

fn main() {
    let files = ["samples/hello_world.o", "samples/zero.o"];
    for file in files {
        println!("{file}:");
        let elf_bytes: Vec<u8> = fs::read(file).unwrap();

        let elf = elf::read_elf(elf_bytes);
        // println!("{elf:?}");
        for header in elf.sections {
            header.dump();
        }

        println!("");
    }
}
