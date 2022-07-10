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

        let mut prev_line = u32::MAX;
        for (index, (ins, &line)) in chunk.code.iter().zip(&chunk.lines).enumerate() {
            if line == prev_line {
                writeln!(self.0, "{index:0>5}    . | {ins:?}")?;
            } else {
                prev_line = line;
                writeln!(self.0, "{index:0>5} {line:>4} | {ins:?}")?;
            }
        }

        Ok(())
    }
}
