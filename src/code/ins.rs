use crate::value::Value;

/// A bytecode instruction.
#[derive(Debug, Clone)]
pub enum Ins {
    /// Return.
    Ret,
    /// Small constant, in which the [Value] is stored inline.
    ///
    /// TODO: Add `BigConst(ArenaIndex)`.
    SmallConst(Value),
    /// Negation operator.
    Neg,
    /// Binary addition.
    Add,
    /// Binary subtraction.
    Sub,
    /// Binary multiplication.
    Mul,
    /// Binary division.
    Div,
}
