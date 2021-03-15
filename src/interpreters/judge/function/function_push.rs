use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax, Type, Types};

impl Interpreter {
  pub(crate) fn function_run(
    &mut self,
    fun: &ast::FunctionAST,
    call: &ast::CallAST,
  ) -> Result<(), result::Error> {
    let argments = call.get_argment();
    let params = fun.get_param();

    if argments.len() != params.len() {
      return Err(result::Error::InterpreterError(format!(
        "function argments error {} {} paramas",
        call.get_name(),
        params.len()
      )));
    }

    for (index, param) in params.iter().enumerate() {
      let types: &ast::Types;
      let name: &str;
      match param {
        Syntax::Var(var) => match var.get_type() {
          Some(t) => {
            types = t;
            name = var.get_name();
          }
          None => {
            return Err(result::Error::InterpreterError(format!(
              "function {} argment {} th not found type impossible interpreter bug",
              call.get_name(),
              index,
            )));
          }
        },
        _ => {
          return Err(result::Error::InterpreterError(format!(
            "function {} argment {} th not found type impossible interpreter bug",
            call.get_name(),
            index,
          )));
        }
      }

      match argments.get(index) {
        Some(args) => match self.function_type_check(args, types, index, name, call) {
          Ok(()) => {}

          Err(e) => {
            return Err(e);
          }
        },

        None => {
          return Err(result::Error::InterpreterError(format!(
            "function {} argment {} th not found impossible interpreter bug",
            call.get_name(),
            index,
          )));
        }
      }
    }

    

    return Ok(());
  }

  fn function_type_check(
    &mut self,
    args: &Syntax,
    types: &Types,
    index: usize,
    name: &str,
    call: &ast::CallAST,
  ) -> Result<(), result::Error> {
    match args {
      Syntax::Bool(_) => {
        if types != &Types::Bool {
          return Err(result::Error::InterpreterError(format!(
            "function {} argment {} th miss matched type",
            call.get_name(),
            index,
          )));
        }

        let mut var = ast::VariableAST::new(name, false, true);
        var.push_node(args.clone());
        match self.push_var(&var) {
          Ok(()) => {}
          Err(e) => {
            return Err(e);
          }
        }
      }
      Syntax::Str(_) => {
        if types != &Types::String {
          return Err(result::Error::InterpreterError(format!(
            "function {} argment {} th miss matched type",
            call.get_name(),
            index,
          )));
        }

        let mut var = ast::VariableAST::new(name, false, true);
        var.push_node(args.clone());
        match self.push_var(&var) {
          Ok(()) => {}
          Err(e) => {
            return Err(e);
          }
        }
      }
      Syntax::Num(_) => {
        if types != &Types::Number {
          return Err(result::Error::InterpreterError(format!(
            "function {} argment {} th miss matched type",
            call.get_name(),
            index,
          )));
        }

        let mut var = ast::VariableAST::new(name, false, true);
        var.push_node(args.clone());
        match self.push_var(&var) {
          Ok(()) => {}
          Err(e) => {
            return Err(e);
          }
        }
      }

      //TODO serch_verでTypesを返すようにする
      Syntax::Var(var) => {
        let serched = self.serch_var(var.get_name());
        match serched.1 {
          Ok(serch) => match serch {
            Some(serch_type) => {
              if &serch_type != types {
                return Err(result::Error::InterpreterError(format!(
                  "function {} argment {} th miss matched type",
                  call.get_name(),
                  index,
                )));
              }

              match serched.0 {
                Some(serch_var) => {
                  let mut var = ast::VariableAST::new(name, false, true);
                  var.push_node(serch_var.clone());
                  match self.push_var(&var) {
                    Ok(()) => {}
                    Err(e) => {
                      return Err(e);
                    }
                  }
                }
                None => {}
              }
            }

            None => {}
          },

          Err(e) => return Err(e),
        }
      }

      _ => {
        return Err(result::Error::InterpreterError(format!(
          "function {} argment {} th not found type impossible interpreter bug",
          call.get_name(),
          index,
        )));
      }
    }
    return Ok(());
  }

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
