use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::lexer::token;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax, VariableAST};
static TOKEN: token::Token = token::Token::new();

impl Interpreter {
  pub(crate) fn variable(&mut self, var: &VariableAST) -> Result<(), result::Error> {
    if var.get_node_len() == 0 {
      return self.push_var(var);
    }

    if var.get_node_len() == 1 {
      return self.substitution_one_node(var);
    }

    return Err(result::Error::InterpreterError(
      "variable node error interpreter bug".to_string(),
    ));
  }

  fn substitution_one_node(&mut self, vars: &VariableAST) -> Result<(), result::Error> {
    match vars.get_node_index(0) {
      Some(var) => {
        match var {
          Syntax::Var(_) => {
            //変数の検索
            return self.is_node_index(vars, var);
          }

          Syntax::Str(_) => {
            return self.is_node_index(vars, var);
          }

          Syntax::Bool(_) => {
            return self.is_node_index(vars, var);
          }

          Syntax::Call(_) => {
            return self.is_node_index(vars, var);
          }

          Syntax::Num(_) => {
            return self.is_node_index(vars, var);
          }

          Syntax::Bin(bin) => {
            let serch = self.serch_var(vars.get_name());
            match serch.0 {
              Some(serched) => match serched {
                Syntax::Num(num) => {
                  let mut result = num.get_num();
                  let mut var_ast = ast::VariableAST::new(vars.get_name(), true, false);

                  if TOKEN._inc == bin.get_token() {
                    result = result + 1;
                    var_ast.push_node(Syntax::Num(ast::NumberAST::new(result)));
                    self.push_var(&var_ast)?;
                    return Ok(());
                  }

                  if TOKEN._dec == bin.get_token() {
                    result = result - 1;
                    var_ast.push_node(Syntax::Num(ast::NumberAST::new(result)));
                    self.push_var(&var_ast)?;
                    return Ok(());
                  }

                  return Err(result::Error::InterpreterError(format!(
                    "{} only dot, increment and decrement operators are supported.",
                    vars.get_name()
                  )));
                }

                Syntax::Var(var) => {
                  if TOKEN._dot == bin.get_token() {
                    match bin.get_node_index(0) {
                      Some(node) => match node {
                        Syntax::Call(call) => match var.serch_functions(call.get_name()) {
                          Some(inner) => {
                            self.function_run(&inner, call, None)?;
                            return Ok(());
                          }
                          None => {
                            return Err(result::Error::InterpreterError(format!(
                              "{} function not found",
                              call.get_name()
                            )));
                          }
                        },
                        _ => {
                          return Ok(());
                        }
                      },
                      None => return Err(result::Error::InterpreterError(format!("value error"))),
                    }
                  }
                  return Err(result::Error::InterpreterError(format!("{} error dot oprator support", var.get_name())));
                }

                _ => {
                  return Err(result::Error::InterpreterError(format!(
                    "{} is missmatched type",
                    vars.get_name()
                  )));
                }
              },
              None => {
                return Err(result::Error::InterpreterError(format!(
                  "{} variable not initialized",
                  vars.get_name()
                )));
              }
            }
          }

          _ => {
            //error
            return Err(result::Error::InterpreterError(
              "Cannot assign to variable".to_string(),
            ));
          }
        }
      }

      None => {
        return Err(result::Error::InterpreterError(
          "Cannot assign to variable".to_string(),
        ));
      }
    }
  }

  fn is_node_index(&mut self, vars: &VariableAST, syn: &Syntax) -> Result<(), result::Error> {
    let mut var = VariableAST::new(vars.get_name(), vars.get_is_mutable(), vars.get_is_def());
    var.push_node(self.formula(syn)?);
    return self.push_var(&var);
  }
}
