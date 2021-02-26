use super::super::interpreter::Interpreter;
use crate::parser::ast::ast::{Syntax};

impl Interpreter {
  pub(crate) fn judge(&mut self, ast: &Syntax) -> Result<(), String> {
    match ast {
      Syntax::Call(call) => {
        return self.call(call);
      }

      Syntax::Bin(bin) => {
        return Err(format!("{} error", bin.get_bin()));
      }

      Syntax::Var(var) => {
        //下の階層にあれば計算してvarにpush
        //なければそのままvar_push
        return self.variable(var);
      }

      Syntax::Str(_) => {
        return Ok(());
      }

      Syntax::Num(_) => {
        return Ok(());
      }

      _ => {
        return Err("error unimplemented ".to_string());
      }
    }
  }
}
