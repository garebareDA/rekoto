use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax};

pub enum FormulaType {
  Bool(bool),
  Strings(String),
  Bumber(i64),
}

pub struct Formula {
  bin_stack: Vec<ast::BinaryAST>,
  stack: Vec<FormulaType>,
}

impl Formula {
  pub fn new() -> Self {
    Self {
      bin_stack: Vec::new(),
      stack: Vec::new(),
    }
  }

  pub fn push_bin(&mut self, bin: ast::BinaryAST) {
    self.bin_stack.push(bin);
  }

  pub fn push_stack(&mut self, stack: FormulaType) {
    self.stack.push(stack);
  }

  pub fn pop_bin(&mut self, index: usize) -> Result<ast::BinaryAST, result::Error> {
    if self.bin_stack.len() > index {
      return Err(result::Error::InterpreterError(
        "pop bin error interpreter bug".to_string(),
      ));
    }

    let bin = self.bin_stack.remove(index);
    return Ok(bin);
  }

  pub fn pop_stack(&mut self, index: usize) -> Result<FormulaType, result::Error> {
    if self.stack.len() > index {
      return Err(result::Error::InterpreterError(
        "pop stack error interpreter bug".to_string(),
      ));
    }

    let formula = self.stack.remove(index);
    return Ok(formula);
  }
}

impl Interpreter {
  pub(crate) fn formula(&mut self, formula: &ast::Syntax) -> Result<Syntax, result::Error> {
    let mut formulas = Formula::new();

    return Err(result::Error::InterpreterError(
      "formula error intepreter bug".to_string(),
    ));
  }
}
