use super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast::Syntax;

impl Interpreter {
  pub(crate) fn judge(&mut self, ast: &Syntax) -> Option<Result<String, result::Error>> {
    match ast {
      Syntax::Call(call) => {
        return Some(self.call(call));
      }

      Syntax::Bin(bin) => {
        return Some(Err(result::Error::InterpreterError(format!(
          "{} binary error",
          bin.get_bin()
        ))));
      }

      Syntax::Var(var) => {
        //下の階層にあれば計算してvarにpush
        //なければそのままvar_push
        match self.variable(var) {
          Ok(()) => {
            return None;
          }

          Err(e) => {
            return Some(Err(e));
          }
        }
      }

      Syntax::Str(_) => {
        return None;
      }

      Syntax::Num(_) => {
        return None;
      }

      _ => {
        return Some(Err(result::Error::InterpreterError(
          "error unimplemented ".to_string(),
        )));
      }
    }
  }
}
