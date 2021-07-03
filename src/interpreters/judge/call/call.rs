use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast::{CallAST, Node, Syntax};

impl Interpreter {
  pub(crate) fn call(
    &mut self,
    call: &CallAST,
  ) -> (
    Option<Result<Option<Syntax>, result::Error>>,
    Option<String>,
  ) {
    if call.get_name() == "print" {
      if call.get_node_len() != 0 {
        return (
          Some(Err(result::Error::InterpreterError(
            "error print cannot be incorporated into formulas".to_string(),
          ))),
          None,
        );
      }

      if call.get_argment_len() != 1 {
        return (
          Some(Err(result::Error::InterpreterError(
            "error print argment 1".to_string(),
          ))),
          None,
        );
      }

      match call.get_argment().get(0) {
        Some(argment) => {
          match self.formula(argment) {
            Ok(result) => match self.print(&result) {
              Ok(ret) => {
                return (None, Some(ret));
              }

              Err(e) => {
                return (Some(Err(e)), None);
              }
            },
            Err(e) => {
              return (Some(Err(e)), None);
            }
          }
        }
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

    match self.serch_fun(call.get_name()) {
      Some(fun) => {
        let result = self.function_run(&fun, call, None);
        return (Some(result), None);
      }
      None => {}
    }

    return (
      Some(Err(result::Error::InterpreterError(format!(
        "not found function {}",
        call.get_name(),
      )))),
      None,
    );
  }
}
