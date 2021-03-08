use super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast::Syntax;

impl Interpreter {
  pub(crate) fn judge(
    &mut self,
    ast: &Syntax,
  ) -> (Option<Result<Syntax, result::Error>>, Option<String>) {
    match ast {
      Syntax::Call(call) => {
        let result = self.call(call);
        if call.get_name() == "print" {
          return (None,result.1);
        }
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

      Syntax::Ifs(ifs) => {
        return (None, None);
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
