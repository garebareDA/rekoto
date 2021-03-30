use super::super::super::interpreter::{Interpreter, InterpreterState};
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax, Type, Types};

impl Interpreter {
  pub(crate) fn function_run(
    &mut self,
    fun: &ast::FunctionAST,
    call: &ast::CallAST,
    add_scope: Option<&ast::VariableAST>
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
        Some(args) => self.function_type_check(args, types, index, name, call)?,
        None => {
          return Err(result::Error::InterpreterError(format!(
            "function {} argment {} th not found impossible interpreter bug",
            call.get_name(),
            index,
          )));
        }
      }
    }

    self.push_state(InterpreterState::Call);
    match fun.get_node_index(0) {
      Some(scope) => match scope {
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
            Some(is_ok) => match is_ok? {
              Some(some_returns) => match some_returns {
                Syntax::Num(_) => {
                  if fun.get_type() != &Some(Types::Number) {
                    return Err(result::Error::InterpreterError(format!(
                      "{} is return value missmatched type",
                      fun.get_name()
                    )));
                  }
                  self.pop_scope();
                  self.pop_state();
                  return Ok(Some(some_returns));
                }
                Syntax::Str(_) => {
                  if fun.get_type() != &Some(Types::String) {
                    return Err(result::Error::InterpreterError(format!(
                      "{} is return value missmatched type",
                      fun.get_name()
                    )));
                  }

                  self.pop_scope();
                  self.pop_state();
                  return Ok(Some(some_returns));
                }
                Syntax::Bool(_) => {
                  if fun.get_type() != &Some(Types::Bool) {
                    return Err(result::Error::InterpreterError(format!(
                      "{} is return value missmatched type",
                      fun.get_name()
                    )));
                  }

                  self.pop_scope();
                  self.pop_state();
                  return Ok(Some(some_returns));
                }

                Syntax::Var(var) => {
                  let serch_var = self.serch_var(var.get_name());
                  match serch_var.1? {
                    Some(types) => {
                      if fun.get_type() != &Some(types) {
                        return Err(result::Error::InterpreterError(format!(
                          "{} is return value missmatched type",
                          fun.get_name()
                        )));
                      }

                      match serch_var.0 {
                        Some(var) => {
                          self.pop_scope();
                          self.pop_state();
                          return Ok(Some(var.clone()));
                        }

                        None => {
                          return Err(result::Error::InterpreterError(format!(
                            "{} is return value variable not found",
                            var.get_name()
                          )));
                        }
                      }
                    }
                    None => {
                      return Err(result::Error::InterpreterError(format!(
                        "{} is return value variable not found",
                        var.get_name()
                      )));
                    }
                  }
                }

                Syntax::Return(_) => {
                  if fun.get_type() != &None {
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
              },
              None => {
                return Err(result::Error::InterpreterError(format!(
                  "break cannot be used in the scope of the function {} function",
                  fun.get_name()
                )));
              }
            },

            None => {
              if fun.get_type() != &None {
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
      },
      None => {
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

  pub(crate) fn function_init(&mut self, root: &ast::RootAST) -> Result<(), result::Error> {
    for ast in root.get_node().iter() {
      match ast {
        Syntax::Fn(fun) => {
          self.push_fun(fun);
        }

        Syntax::Var(var) => {
          //下の階層にあれば計算してvarにpush
          //なければそのままvar_push
          self.variable(var)?;
        }

        Syntax::Import(import) => match import.get_node_index(0) {
          Some(inner) => match inner {
            Syntax::Str(strs) => {
              self.push_var(&self.import(strs.get_str())?)?;
            }

            _ => {
              return Err(result::Error::InterpreterError(
                "please specify import as a string ".to_string(),
              ));
            }
          },
          None => {
            return Err(result::Error::InterpreterError(format!("import error")));
          }
        },

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
