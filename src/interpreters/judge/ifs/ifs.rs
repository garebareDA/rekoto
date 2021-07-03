use super::super::super::interpreter::{Interpreter, InterpreterState};
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
      Ok(bools) => {
        return self.ifs_judge(bools, ifs, "if");
      }
      Err(e) => (Some(Err(e)), None),
    }
  }

  pub(crate) fn ifs_judge<T: Node>(
    &mut self,
    bools: Syntax,
    ifs: &Box<T>,
    message: &str,
  ) -> (
    Option<Result<Option<Syntax>, result::Error>>,
    Option<String>,
  ) {
    match bools {
      Syntax::Bool(bools) => {
        if bools.get_bool() {
          self.pop_state();
          return self.ifs_scope(ifs, message);
        } else {
          return (None, None);
        }
      }

      _ => {
        return (
          Some(Err(result::Error::InterpreterError(format!(
            "{} is judge not bool",
            message
          )))),
          None,
        )
      }
    }
  }

  pub(crate) fn ifs_scope<T: Node>(
    &mut self,
    ifs: &Box<T>,
    message: &str,
  ) -> (
    Option<Result<Option<Syntax>, result::Error>>,
    Option<String>,
  ) {
    match ifs.get_node_index(0) {
      Some(scopes) => match scopes {
        Syntax::Scope(_) => {
          let scope = self.judge(scopes);
          self.push_state(InterpreterState::IfDone);
          return scope;
        }
        _ => {
          return (
            Some(Err(result::Error::InterpreterError(format!(
              "{} scope not found error",
              message
            )))),
            None,
          );
        }
      },
      None => {
        return (
          Some(Err(result::Error::InterpreterError(format!(
            "{} scope not found error",
            message
          )))),
          None,
        );
      }
    }
  }
}
