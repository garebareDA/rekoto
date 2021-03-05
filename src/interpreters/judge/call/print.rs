use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast::Syntax;
use std::io;
use std::io::Write;

impl Interpreter {
  pub(crate) fn print(&mut self, argment: &Syntax) -> Result<String, result::Error> {
    match argment {
      Syntax::Str(strs) => {
        self.print_out(strs.get_str()).unwrap();
        return Ok(strs.get_str().to_string());
      }

      Syntax::Num(num) => {
        self.print_out(&num.get_num().to_string()).unwrap();
        return Ok(num.get_num().to_string());
      }

      Syntax::Bool(bools) => {
        self.print_out(&bools.get_bool().to_string()).unwrap();
        return Ok(bools.get_bool().to_string());
      }

      Syntax::Var(var) => match self.serch_var(var.get_name()) {
        Some(vars) => match vars {
          Syntax::Str(strs) => {
            self.print_out(strs.get_str()).unwrap();
            return Ok(strs.get_str().to_string());
          }

          Syntax::Num(num) => {
            self.print_out(&num.get_num().to_string()).unwrap();
            return Ok(num.get_num().to_string());
          }

          Syntax::Bool(bools) => {
            self.print_out(&bools.get_bool().to_string()).unwrap();
            return Ok(bools.get_bool().to_string());
          }

          _ => {
            return Err(result::Error::InterpreterError(
              "error print variable".to_string(),
            ))
          }
        },
        None => {
          return Err(result::Error::InterpreterError(
            "error print variable".to_string(),
          ))
        }
      },

      _ => return Err(result::Error::InterpreterError("error print".to_string())),
    }
  }

  pub(crate) fn print_out(&self, string: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(format!("{}\n", string).as_bytes())?;
    return Ok(());
  }
}