use crate::{
    code::{Chunk, Ins},
    common::Result,
    value::Value,
};

pub struct Vm<'c> {
    chunk: &'c Chunk,
    ip: usize,
    stack: Vec<Value>,
}

// Actual implementation.
impl Vm<'_> {
    pub fn run(&mut self) -> Result<()> {
        loop {
            match self.read() {
                Ins::Ret => {
                    let val = self.pop();
                    println!("{val:?}");
                    return Ok(());
                }
                Ins::SmallConst(val) => {
                    self.push(val);
                }
                Ins::Neg => {
                    let res = -self.pop().into_number().expect("type error");
                    self.push(Value::Number(res))
                }
            }
        }
    }
}

// Other public interface methods.
impl<'c> Vm<'c> {
    pub fn new(chunk: &'c Chunk) -> Vm<'c> {
        Vm {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(128),
        }
    }
}

// Utilities.
impl Vm<'_> {
    /// Advances the instruction pointer returning a clone of the previously pointed-to [Ins].
    #[inline(always)]
    fn read(&mut self) -> Ins {
        let prev = self.chunk.code[self.ip].clone();
        self.ip += 1;
        prev
    }

    /// Pushes a value into the stack.
    #[inline(always)]
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    /// Removes a value out of the stack and returns it.
    ///
    /// Panics if the stack is empty.
    #[inline(always)]
    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Called pop on empty stack")
    }
}
