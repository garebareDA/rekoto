use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax};

impl Interpreter {
  pub(crate) fn function_run(&mut self, fun: &ast::FunctionAST) {}

  pub(crate) fn function_init(&mut self, root: &ast::RootAST) -> Result<(), result::Error> {
    for ast in root.get_node().iter() {
      match ast {
        Syntax::Fn(fun) => {
          self.push_fun(fun);
        }

        Syntax::Var(var) => {
          //下の階層にあれば計算してvarにpush
          //なければそのままvar_push
          match self.variable(var) {
            Ok(()) => {
              return Ok(());
            }
            Err(e) => {
              return Err(e);
            }
          }
        }

        _ => {
          return Err(result::Error::InterpreterError(
            "the syntax is not written inside the function".to_string(),
          ));
        }
      }
    }
    return Ok(());
  }
}
