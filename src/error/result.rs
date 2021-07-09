use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
  InterpreterError(String),
  SyntaxError(String),
  FileReadError(String),
  CompileError(String),
}

impl fmt::Display for Error{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::InterpreterError(i) => writeln!(f, "Runtime Error: {}", i),
      Error::SyntaxError(s) => writeln!(f, "Sytax Error: {}", s),
      Error::FileReadError(e) => writeln!(f, "File Read Error: {}", e),
      Error::CompileError(e) => writeln!(f, "Compile Error: {}", e),
    }
  }
}

impl error::Error for Error{}