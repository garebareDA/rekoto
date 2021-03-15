use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax, Type, Types};

impl Interpreter {
  pub(crate) fn function_run(
    &mut self,
    fun: &ast::FunctionAST,
    call: &ast::CallAST,
  ) -> Result<Option<Syntax>, result::Error> {
    let argments = call.get_argment();
    let params = fun.get_param();

    //argmentsにformula
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

    match fun.get_node_index(0) {
      Some(scope) => match scope {
        Syntax::Scope(_) => {
          let scopes = self.judge(scope);
          match scopes.0 {
            Some(is_ok) => match is_ok {
              Ok(returns) => match returns {
                Some(some_returns) => match some_returns {
                  Syntax::Num(_) => {
                    if fun.get_type() != &Some(Types::Number) {
                      return Err(result::Error::InterpreterError(format!(
                        "{} is return value missmatched type",
                        fun.get_name()
                      )));
                    }

                    return Ok(Some(some_returns));
                  }
                  Syntax::Str(_) => {
                    if fun.get_type() != &Some(Types::String) {
                      return Err(result::Error::InterpreterError(format!(
                        "{} is return value missmatched type",
                        fun.get_name()
                      )));
                    }

                    return Ok(Some(some_returns));
                  }
                  Syntax::Bool(_) => {
                    if fun.get_type() != &Some(Types::Bool) {
                      return Err(result::Error::InterpreterError(format!(
                        "{} is return value missmatched type",
                        fun.get_name()
                      )));
                    }

                    return Ok(Some(some_returns));
                  }

                  Syntax::Var(var) => {
                    let serch_var = self.serch_var(var.get_name());
                    match serch_var.1 {
                      Ok(serch_type) => match serch_type {
                        Some(types) => {
                          if fun.get_type() != &Some(types) {
                            return Err(result::Error::InterpreterError(format!(
                              "{} is return value missmatched type",
                              fun.get_name()
                            )));
                          }

                          match serch_var.0 {
                            Some(var) => {
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
                      },

                      Err(e) => {
                        return Err(e);
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
              Err(e) => {
                return Err(e);
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
