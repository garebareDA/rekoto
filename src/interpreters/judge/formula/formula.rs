use super::super::super::interpreter::Interpreter;
use crate::parser::ast::ast::{Node, Syntax};
use crate::parser::ast::ast;

impl Interpreter{
  pub(crate) fn formula(&mut self, var: &ast::NumberAST) -> Result<(), String> {
    return Ok(());
  }
}