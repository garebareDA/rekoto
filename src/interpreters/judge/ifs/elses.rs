use super::super::super::interpreter::{Interpreter, InterpreterState};
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::Syntax;

impl Interpreter {
  pub(crate) fn elses(
    &mut self,
    elses: &Box<ast::ElseAST>,
  ) -> (
    Option<Result<Option<Syntax>, result::Error>>,
    Option<String>,
  ) {
    match self.else_check("else") {
      Ok(o) => match o {
        Some(()) => {}
        None => return (None, None),
      },

      Err(e) => {
        return (Some(Err(e)), None);
      }
    }

    self.pop_state();
    return self.ifs_scope(elses, "else");
  }

  pub(crate) fn else_check(&self, m:&str) -> Result<Option<()>, result::Error> {
    match self.get_last_state() {
      Some(state) => {
        if state == &InterpreterState::IfDone {
          return Ok(None);
        }

        if state != &InterpreterState::If {
          return Err(result::Error::InterpreterError(
            format!("not found if dont used {}", m),
          ));
        } else {
          return Ok(Some(()));
        }
      }
      None => {}
    }

    return Err(result::Error::InterpreterError(
      format!("not found if dont used {}", m),
    ));
  }
}
