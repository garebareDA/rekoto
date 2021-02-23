use super::super::interpreter::Interpreter;
use crate::parser::ast::ast;

impl Interpreter {
  pub(crate) fn judge(&self, ast: &ast::Syntax) -> Result<(), String> {
    match ast {
      _ => {
        return Err("error unimplemented ".to_string());
      }
    }
  }
}
