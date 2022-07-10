use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    /// Errors that happen at compile time.
    Compiler,
    /// Errors that happen at run time.
    Runtime(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // FIXME.
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
