use std::fmt::{self, Display};

use crate::code::Chunk;

// TODO: Maybe expose `ChunkDisassembler` and `IndexedInsDisassembler` if, in the future, one would like to have more methods in those types.

pub fn disassemble_chunk<'a>(chunk: &'a Chunk, name: &'a str) -> impl Display + 'a {
    struct ChunkDisassembler<'a>(&'a Chunk, &'a str);

    impl Display for ChunkDisassembler<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let ChunkDisassembler(chunk, name) = *self;
            writeln!(f, "=== {name} ===")?;
            for i in 0..chunk.code.len() {
                writeln!(f, "{}", disassemble_indexed_ins(chunk, i))?;
            }
            Ok(())
        }
    }

    ChunkDisassembler(chunk, name)
}

pub fn disassemble_indexed_ins(chunk: &Chunk, index: usize) -> impl Display + '_ {
    struct IndexedInsDisassembler<'a>(&'a Chunk, usize);

    impl Display for IndexedInsDisassembler<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let IndexedInsDisassembler(chunk, index) = *self;

            // Index.
            write!(f, "{index:0>5} ")?;

            // Line information.
            let line = chunk.lines[index];
            if index > 0 && line == chunk.lines[index - 1] {
                write!(f, "{:>4} | ", '.')?;
            } else {
                write!(f, "{line:>4} | ")?;
            }

            // Actual instruction.
            let ins = &chunk.code[index];
            write!(f, "{ins:?}")
        }
    }

    IndexedInsDisassembler(chunk, index)
}
