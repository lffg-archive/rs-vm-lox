use crate::code::Ins;

/// A bytecode chunk.
#[derive(Debug, Default)]
pub struct Chunk {
    pub(crate) code: Vec<Ins>,
    pub(crate) lines: Vec<u32>,
}

impl Chunk {
    pub fn write(&mut self, ins: Ins, line: u32) {
        debug_assert_eq!(self.code.len(), self.lines.len());

        self.code.push(ins);
        self.lines.push(line);
    }
}
