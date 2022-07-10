use std::io::stdout;

use lox::{
    code::{Chunk, Disassembler, Ins},
    value::Value,
};

fn main() {
    let mut chunk = Chunk::default();
    chunk.write(Ins::Const(Value::Number(1.)), 1);
    chunk.write(Ins::Const(Value::Number(2.)), 1);
    chunk.write(Ins::Ret, 2);

    Disassembler::new(&mut stdout())
        .disassemble_chunk(&chunk, "Main Chunk")
        .unwrap();
}
