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
}
