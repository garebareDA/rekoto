use super::super::super::interpreter::{Interpreter, InterpreterState};
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax, Type, Types};

impl Interpreter {
  pub(crate) fn function_run(
    &mut self,
    fun: &ast::FunctionAST,
    call: &ast::CallAST,
    add_scope: Option<&ast::VariableAST>,
  ) -> Result<Option<Syntax>, result::Error> {
    let argments = call.get_argment();
    let params = fun.get_param();

    if argments.len() != params.len() {
      return Err(result::Error::InterpreterError(format!(
        "function argments error {} {} paramas",
        call.get_name(),
        params.len()
      )));
    }

    self.push_scope();
    match add_scope {
      Some(add) => {
        for var in add.get_variables().iter() {
          self.push_var(var)?;
        }

        for fun in add.get_functions().iter() {
          self.push_fun(fun);
        }
      }
      None => {}
    }

    for (index, param) in params.iter().enumerate() {
      let types: &ast::Types;
      let name: &str;
      match param {
        Syntax::Var(var) => {
          types = var
            .get_type()
            .ok_or(result::Error::InterpreterError(format!(
              "function {} argment {} th not found type impossible interpreter bug",
              call.get_name(),
              index,
            )))?;
          name = var.get_name();
        }
        _ => {
          return Err(result::Error::InterpreterError(format!(
            "function {} argment {} th not found type impossible interpreter bug",
            call.get_name(),
            index,
          )));
        }
      }

      let args = argments
        .get(index)
        .ok_or(result::Error::InterpreterError(format!(
          "function {} argment {} th not found impossible interpreter bug",
          call.get_name(),
          index,
        )))?;
      self.function_type_check(args, &types, index, name, call)?;
    }

    self.push_state(InterpreterState::Call);
    let scope = fun
      .get_node_index(0)
      .ok_or(result::Error::InterpreterError(format!(
        "{} function not found scope",
        fun.get_name()
      )))?;

    match scope {
      Syntax::Scope(s) => {
        let scopes = self.scope(s);
        match self.get_last_state() {
          Some(state) => {
            if state == &InterpreterState::IfDone {
              self.pop_state();
            }
          }
          None => {}
        }

        match scopes.0 {
          Some(is_ok) => {
            let some_return = is_ok?.ok_or(result::Error::InterpreterError(format!(
              "break cannot be used in the scope of the function {} function",
              fun.get_name()
            )))?;

            match some_return {
              Syntax::Num(_) => {
                if fun.get_type() != Some(&Types::Number) {
                  return Err(result::Error::InterpreterError(format!(
                    "{} is return value missmatched type",
                    fun.get_name()
                  )));
                }
                self.pop_scope();
                self.pop_state();
                return Ok(Some(some_return));
              }
              Syntax::Str(_) => {
                if fun.get_type() != Some(&Types::String) {
                  return Err(result::Error::InterpreterError(format!(
                    "{} is return value missmatched type",
                    fun.get_name()
                  )));
                }

                self.pop_scope();
                self.pop_state();
                return Ok(Some(some_return));
              }
              Syntax::Bool(_) => {
                if fun.get_type() != Some(&Types::Bool) {
                  return Err(result::Error::InterpreterError(format!(
                    "{} is return value missmatched type",
                    fun.get_name()
                  )));
                }

                self.pop_scope();
                self.pop_state();
                return Ok(Some(some_return));
              }

              Syntax::Var(var) => {
                let serched_var = self.serch_var(var.get_name());
                let serched_var_type =
                  serched_var
                    .1?
                    .ok_or(result::Error::InterpreterError(format!(
                      "{} is return value variable not found",
                      var.get_name()
                    )))?;

                let serched_var_value =
                  serched_var
                    .0
                    .ok_or(result::Error::InterpreterError(format!(
                      "{} is return value variable not found",
                      var.get_name()
                    )))?;

                if fun.get_type() != Some(&serched_var_type) {
                  return Err(result::Error::InterpreterError(format!(
                    "{} is return value missmatched type",
                    fun.get_name()
                  )));
                }

                self.pop_scope();
                self.pop_state();
                return Ok(Some(serched_var_value));
              }

              Syntax::Return(_) => {
                if fun.get_type() != None {
                  return Err(result::Error::InterpreterError(format!(
                    "{} is return value missmatched type",
                    fun.get_name()
                  )));
                }

                self.pop_scope();
                self.pop_state();
                return Ok(None);
              }
              _ => {
                return Err(result::Error::InterpreterError(format!(
                  "cannot be specified as a return value",
                )));
              }
            }
          }

          None => {
            if fun.get_type() != None {
              return Err(result::Error::InterpreterError(format!(
                "{} is return value missmatched type",
                fun.get_name()
              )));
            }
            return Ok(None);
          }
        }
      }

      _ => {
        return Err(result::Error::InterpreterError(format!(
          "{} function not found scope",
          fun.get_name()
        )))
      }
    }
  }

  fn function_type_check(
    &mut self,
    args: &Syntax,
    types: &Types,
    index: usize,
    name: &str,
    call: &ast::CallAST,
  ) -> Result<(), result::Error> {
    match self.formula(args)? {
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
        self.push_var(&var)?;
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
        self.push_var(&var)?;
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
        self.push_var(&var)?;
      }

      Syntax::Var(var) => {
        let serched = self.serch_var(var.get_name());
        match serched.1? {
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
                self.push_var(&var)?;
              }
              None => {}
            }
          }
          None => {}
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
}
