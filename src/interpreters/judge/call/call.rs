use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast::{CallAST, Node};

impl Interpreter {
  pub(crate) fn call(&mut self, call: &CallAST) -> Result<String, result::Error> {
    let node_len = call.get_node_len();
    let argment_len = call.get_argment_len();
    let argment = call.get_argment();
    let name = call.get_name();

    if name == "print" {
      if node_len != 0 {
        return Err(result::Error::InterpreterError(
          "error print cannot be incorporated into formulas".to_string(),
        ));
      }

      if argment_len != 1 {
        return Err(result::Error::InterpreterError(
          "error print argment 1".to_string(),
        ));
      }

      match argment.get(0) {
        Some(argment) => {
          return self.print(argment);
        }
        None => {
          return Err(result::Error::InterpreterError(
            "error print not argment".to_string(),
          ));
        }
      }
    }

    return Err(result::Error::InterpreterError(format!(
      "not found function {}",
      name
    )));
  }
}
