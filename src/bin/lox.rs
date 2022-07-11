use lox::{
    code::{disassemble_chunk, Chunk, Ins},
    value::Value,
    vm::Vm,
};

fn main() {
    let mut chunk = Chunk::default();
    chunk.write(Ins::SmallConst(Value::Number(2.)), 1);
    chunk.write(Ins::Neg, 1);
    chunk.write(Ins::Ret, 2);

    if cfg!(feature = "debug") {
        println!("{}", disassemble_chunk(&chunk, "Main Chunk"));

        println!("Running now...");
    }

    let mut vm = Vm::new(&chunk);
    vm.run().unwrap();
}
