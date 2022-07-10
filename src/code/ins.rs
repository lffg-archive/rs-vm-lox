use crate::value::Value;

/// A bytecode instruction.
#[derive(Debug, Clone)]
pub enum Ins {
    /// Return.
    Ret,
    /// Constant.
    Const(Value),
    /// Negation operator.
    Neg,
}
