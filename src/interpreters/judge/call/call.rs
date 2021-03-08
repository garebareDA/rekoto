use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast::{CallAST, Node, Syntax};

impl Interpreter {
  pub(crate) fn call(
    &mut self,
    call: &CallAST,
  ) -> (Option<Result<Option<Syntax>, result::Error>>, Option<String>) {
    let node_len = call.get_node_len();
    let argment_len = call.get_argment_len();
    let argment = call.get_argment();
    let name = call.get_name();

    if name == "print" {
      if node_len != 0 {
        return (
          Some(Err(result::Error::InterpreterError(
            "error print cannot be incorporated into formulas".to_string(),
          ))),
          None,
        );
      }

      if argment_len != 1 {
        return (
          Some(Err(result::Error::InterpreterError(
            "error print argment 1".to_string(),
          ))),
          None,
        );
      }

      match argment.get(0) {
        Some(argment) => match self.print(argment) {
          Ok(ret) => {
            return (None, Some(ret));
          }

          Err(e) => {
            return (Some(Err(e)), None);
          }
        },
        None => {
          return (
            Some(Err(result::Error::InterpreterError(
              "error print not argment".to_string(),
            ))),
            None,
          );
        }
      }
    }

    return (
      Some(Err(result::Error::InterpreterError(format!(
        "not found function {}",
        name
      )))),
      None,
    );
  }
}
