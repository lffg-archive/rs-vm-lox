use crate::{
    code::{Chunk, Ins},
    common::{Error, Result},
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
                Ins::Neg => self.un_op("unary `-`", |v| match v {
                    Value::Number(n) => Some(Value::Number(-n)),
                    _ => None,
                })?,
                Ins::Add => self.bin_op("binary `+`", |a, b| match (a, b) {
                    (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
                    _ => None,
                })?,
                Ins::Sub => self.bin_op("binary `-`", |a, b| match (a, b) {
                    (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
                    _ => None,
                })?,
                Ins::Mul => self.bin_op("binary `*`", |a, b| match (a, b) {
                    (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
                    _ => None,
                })?,
                Ins::Div => self.bin_op("binary `/`", |a, b| match (a, b) {
                    (Value::Number(a), Value::Number(b)) => Some(Value::Number(a / b)),
                    _ => None,
                })?,
            }
        }
    }

    #[inline]
    fn un_op(&mut self, op: &str, r#fn: impl FnOnce(Value) -> Option<Value>) -> Result<()> {
        let v = self.pop();
        let ty = v.type_name();
        match r#fn(v) {
            Some(val) => {
                self.push(val);
                Ok(())
            }
            None => Err(Error::rt(format!("Can not apply {op} over `{ty}`"))),
        }
    }

    #[inline]
    fn bin_op(&mut self, op: &str, r#fn: impl FnOnce(Value, Value) -> Option<Value>) -> Result<()> {
        let b = self.pop();
        let a = self.pop();
        let ty_a = a.type_name();
        let ty_b = b.type_name();
        match r#fn(a, b) {
            Some(val) => {
                self.push(val);
                Ok(())
            }
            None => Err(Error::rt(format!(
                "Can not apply {op} over `{ty_a}` and `{ty_b}`"
            ))),
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
