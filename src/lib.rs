/// AST representation.
pub mod ast;
/// Bytecode representation.
pub mod code;
/// Common.
pub mod common;
/// Compiler implementation. Takes an AST and emits bytecode.
pub mod compiler;
/// Parser implementation. Takes text and constructs the AST.
pub mod parser;
/// Value representation.
pub mod value;
/// Virtual machine (VM) implementation. Takes the bytecode and executes it.
pub mod vm;
