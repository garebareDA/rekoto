use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax};

impl Interpreter {
  pub(crate) fn fors(
    &mut self,
    fors: &Box<ast::ForsAST>,
  ) -> (
    Option<Result<Option<Syntax>, result::Error>>,
    Option<String>,
  ) {
    let mut message = None;

    match fors.get_init() {
      Syntax::Var(_) => {
        self.judge(fors.get_init());
      }
      _ => {
        return (
          Some(Err(result::Error::InterpreterError(
            "for init error".to_string(),
          ))),
          None,
        );
      }
    }

    loop {
      match self.formula(fors.get_judge()) {
        Ok(formula) => match formula {
          Syntax::Bool(bools) => {
            if bools.get_bool() == false {
              return (None, message);
            }
          }
          _ => {
            return (
              Some(Err(result::Error::InterpreterError(
                "for judge error not a bool".to_string(),
              ))),
              None,
            );
          }
        },

        Err(e) => {
          return (Some(Err(e)), None);
        }
      }

      match fors.get_node_index(0) {
        Some(scope) => match scope {
          Syntax::Scope(_) => {
            let scopes = self.judge(scope);
            match scopes.0 {
              Some(is_ok) => {
                return (Some(is_ok), scopes.1);
              }

              None => {
                message = scopes.1;
              }
            }
          }
          _ => {
            return (
              Some(Err(result::Error::InterpreterError(
                "for error not found scope".to_string(),
              ))),
              None,
            );
          }
        },

        _ => {
          return (
            Some(Err(result::Error::InterpreterError(
              "for error not found scope".to_string(),
            ))),
            None,
          );
        }
      }

      let formula = self.judge(fors.get_counter());
      match formula.0 {
        Some(s) => match s {
          Ok(_) => {}
          Err(e) => {
            return (
              Some(Err(result::Error::InterpreterError(
                format!("for error formula \n {}", e),
              ))),
              formula.1,
            );
          }
        },
        None => {}
      }
    }
  }
}
