use std::io::{Result, Write};

use crate::code::Chunk;

pub struct Disassembler<'w>(&'w mut dyn Write);

impl<'w> Disassembler<'w> {
    pub fn new(writer: &'w mut dyn Write) -> Disassembler<'w> {
        Self(writer)
    }
}

impl Disassembler<'_> {
    pub fn disassemble_chunk(&mut self, chunk: &Chunk, name: &str) -> Result<()> {
        writeln!(self.0, "=== {name} ===")?;

        for i in 0..chunk.code.len() {
            self.disassemble_ins(chunk, i)?;
        }

        Ok(())
    }

    pub fn disassemble_ins(&mut self, chunk: &Chunk, index: usize) -> Result<()> {
        // Index.
        write!(self.0, "{index:0>5} ")?;

        // Line information.
        let line = chunk.lines[index];
        if index > 0 && line == chunk.lines[index - 1] {
            write!(self.0, "{:>4} | ", '.')?;
        } else {
            write!(self.0, "{line:>4} | ")?;
        }

        // Actual instruction.
        let ins = &chunk.code[index];
        writeln!(self.0, "{ins:?}")
    }
}
