use crate::{
    code::{Chunk, Disassembler, Ins},
    common::Result,
    value::Value,
};

pub struct Vm<'c, 'd> {
    chunk: &'c Chunk,
    ip: usize,
    stack: Vec<Value>,
    disassembler: Option<&'d mut Disassembler<'d>>,
}

// Actual implementation.
impl Vm<'_, '_> {
    pub fn run(&mut self) -> Result<()> {
        use Ins::*;

        loop {
            if cfg!(feature = "debug") {
                self.debug_stack();
                self.disassemble_curr();
            }

            match read!(self) {
                Ret => {
                    let val = self.pop();
                    println!("{val:?}");
                    return Ok(());
                }
                Const(val) => {
                    self.push(val.clone());
                }
                Neg => {
                    let res = -self.pop().into_number().expect("type error");
                    self.push(Value::Number(res))
                }
            }
        }
    }
}

impl<'c, 'd> Vm<'c, 'd> {
    pub fn new(chunk: &'c Chunk) -> Vm<'c, 'd> {
        Vm {
            chunk,
            ip: 0,
            disassembler: None,
            stack: Vec::with_capacity(128),
        }
    }

    pub fn set_disassembler(&mut self, disassembler: &'d mut Disassembler<'d>) {
        self.disassembler = Some(disassembler);
    }
}

impl Vm<'_, '_> {
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

    /// Unsafely removes a value out of the stack and returns it.
    ///
    /// SAFETY: The caller must ensure the stack is not empty.
    #[inline(always)]
    #[allow(dead_code)]
    unsafe fn pop_unchecked(&mut self) -> Value {
        self.stack.pop().unwrap_unchecked()
    }
}

/// Advances the instruction pointer, returning a reference to the previously pointed-to [Instruction].
///
/// This have to be a macro to escape the borrow checker madness. ;)
macro_rules! read {
    ($self:expr) => {{
        let old = &$self.chunk.code[$self.ip];
        $self.ip += 1;
        old
    }};
}
use read;

impl Vm<'_, '_> {
    /// Disassembles the current instruction.
    fn disassemble_curr(&mut self) {
        self.disassembler
            .as_mut()
            // SAFETY: Running a Vm in debug mode without setting a disassembly is a programmer's fault.
            .expect("Must define `disassembler` to run `Vm` in debug mode")
            .disassemble_ins(&self.chunk, self.ip)
            // SAFETY: We can't do anything if writing to `disassembler` output sink fails.
            .unwrap();
    }

    /// Prints the stack to the stdout.
    fn debug_stack(&self) {
        print!("           | [ ");
        for value in self.stack.iter().rev() {
            print!("{value:?} ");
        }
        println!("]");
    }
}
