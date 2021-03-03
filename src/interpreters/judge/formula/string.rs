use super::super::super::interpreter::Interpreter;
use crate::parser::ast::ast::{Node, Syntax};
use crate::parser::ast::ast;

impl Interpreter{
  pub(crate) fn string(&mut self, var: &ast::Syntax) -> Result<(), String> {
    return Ok(());
  }
}