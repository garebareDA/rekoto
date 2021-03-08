use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax};

impl Interpreter {
  pub(crate) fn ifs(
    &mut self,
    ifs: &Box<ast::IfsAST>,
  ) -> (
    Option<Result<Option<Syntax>, result::Error>>,
    Option<String>,
  ) {
    match self.formula(ifs.get_judge()) {
      Ok(bools) => match bools {
        Syntax::Bool(bools) => {
          if bools.get_bool() {
            self.pop_state();
            match ifs.get_node_index(0) {
              Some(scope) => match scope {
                Syntax::Scope(scope) => {
                  return self.scope(scope);
                }
                _ => {
                  return (
                    Some(Err(result::Error::InterpreterError(
                      "if scope not found error".to_string(),
                    ))),
                    None,
                  );
                }
              },
              None => {
                return (
                  Some(Err(result::Error::InterpreterError(
                    "if scope not found error".to_string(),
                  ))),
                  None,
                );
              }
            }
          } else {
            return (None, None);
          }
        }

        _ => {
          return (
            Some(Err(result::Error::InterpreterError(
              "if is judge not bool".to_string(),
            ))),
            None,
          )
        }
      },
      Err(e) => (Some(Err(e)), None),
    }
  }
}
