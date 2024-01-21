toy ELF64 linker written in rust

## resources used
- [wikipedia overview](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
- [elf32 abi specification](https://www.sco.com/developers/devspecs/gabi41.pdf)
- [elf64 abi specification](https://uclibc.org/docs/elf-64-gen.pdf)
- [linker blog post collection](https://www.airs.com/blog/page/4?s=linkers)

## Basic Linker Operation

At this point we already know enough to understand the basic steps used by every linker.
- Read the input object files. Determine the length and type of the contents. Read the symbols.
- Build a symbol table containing all the symbols, linking undefined symbols to their definitions.
- Decide where all the contents should go in the output executable file, which means deciding where they should go in memory when the program runs.
- Read the contents data and the relocations. Apply the relocations to the contents. Write the result to the output file.
- Optionally write out the complete symbol table with the final values of the symbols.
