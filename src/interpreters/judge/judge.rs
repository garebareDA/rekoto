use super::super::interpreter::{Interpreter, InterpreterState};
use crate::error::result;
use crate::parser::ast::ast::{Node, Syntax};

impl Interpreter {
  pub(crate) fn judge(
    &mut self,
    ast: &Syntax,
  ) -> (
    Option<Result<Option<Syntax>, result::Error>>,
    Option<String>,
  ) {
    match ast {
      Syntax::Elif(_) => {}
      Syntax::Else(_) => {}
      _ => match self.get_last_state() {
        Some(state) => {
          if state == &InterpreterState::IfDone {
            self.pop_state();
          }
        }
        None => {}
      },
    }

    match ast {
      Syntax::Call(call) => {
        self.push_state(InterpreterState::Call);
        let result = self.call(call);
        if call.get_name() == "print" {
          return (None, result.1);
        }
        self.pop_state();
        return (result.0, None);
      }

      Syntax::Bin(bin) => {
        return (
          Some(Err(result::Error::InterpreterError(format!(
            "{} binary error",
            bin.get_bin()
          )))),
          None,
        );
      }

      Syntax::Var(var) => {
        //下の階層にあれば計算してvarにpush
        //なければそのままvar_push
        match self.variable(var) {
          Ok(()) => {
            return (None, None);
          }

          Err(e) => {
            return (Some(Err(e)), None);
          }
        }
      }

      Syntax::Ifs(ifs) => {
        self.push_state(InterpreterState::If);
        return self.ifs(ifs);
      }

      Syntax::Elif(elses) => {
        return self.elif(elses);
      }

      Syntax::Else(elses) => {
        return self.elses(elses);
      }

      Syntax::For(fors) => {
        self.push_state(InterpreterState::For);
        let fors = self.fors(fors);
        self.pop_state();
        return fors;
      }

      Syntax::Scope(scope) => {
        self.push_scope();
        let result = self.scope(scope);
        self.pop_scope();
        return result;
      }

      Syntax::Return(ret) => match ret.get_node_index(0) {
        //TODO formulaを噛ませる
        Some(syntax) => {
          return (Some(Ok(Some(syntax.clone()))), None);
        }
        None => {
          return (Some(Ok(Some(Syntax::Return(ret.clone())))), None);
        }
      },

      Syntax::Break => {
        return (Some(Ok(None)), None);
      }

      Syntax::Str(_) => {
        return (None, None);
      }

      Syntax::Num(_) => {
        return (None, None);
      }

      _ => {
        return (
          Some(Err(result::Error::InterpreterError(
            "error unimplemented ".to_string(),
          ))),
          None,
        );
      }
    }
  }
}
