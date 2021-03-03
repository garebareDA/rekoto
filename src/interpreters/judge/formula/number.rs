use super::super::super::interpreter::Interpreter;
use crate::parser::ast::ast::{Node, Syntax};
use crate::parser::ast::ast;
use crate::error::result;

impl Interpreter{
  pub(crate) fn number(&mut self, number: &ast::NumberAST) -> Result<Syntax, result::Error> {
    
    return Err(result::Error::InterpreterError("number error interpreter bug".to_string()));
  }
}