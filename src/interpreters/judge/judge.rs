use super::super::interpreter::Interpreter;
use crate::parser::ast::ast::{Syntax};

impl Interpreter {
  pub(crate) fn judge(&self, ast: &Syntax) -> Result<(), String> {
    match ast {
      Syntax::Call(call) => {
        return self.call(call);
      }

      _ => {
        return Err("error unimplemented ".to_string());
      }
    }
  }
}
