use std::io::stdout;

use lox::{
    code::{Chunk, Disassembler, Ins},
    value::Value,
    vm::Vm,
};

fn main() {
    let mut chunk = Chunk::default();
    chunk.write(Ins::Const(Value::Number(2.)), 1);
    chunk.write(Ins::Neg, 1);
    chunk.write(Ins::Ret, 2);

    let stdout = &mut stdout();
    let mut disassembler = Disassembler::new(stdout);

    if cfg!(feature = "debug") {
        disassembler
            .disassemble_chunk(&chunk, "Main Chunk")
            .unwrap();

        println!("\nRunning now...");
    }

    let mut vm = Vm::new(&chunk);
    vm.set_disassembler(&mut disassembler);
    vm.run().unwrap();
}
