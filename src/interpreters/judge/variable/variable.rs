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
    let var_node = vars
      .get_node_index(0)
      .ok_or(result::Error::InterpreterError(
        "Cannot assign to variable".to_string(),
      ))?;
    match var_node {
      Syntax::Var(_) => {
        //変数の検索
        return self.is_node_index(vars, var_node);
      }

      Syntax::Str(_) => {
        return self.is_node_index(vars, var_node);
      }

      Syntax::Bool(_) => {
        return self.is_node_index(vars, var_node);
      }

      Syntax::Call(_) => {
        return self.is_node_index(vars, var_node);
      }

      Syntax::Num(_) => {
        return self.is_node_index(vars, var_node);
      }

      Syntax::Bin(bin) => {
        let serched_var = self.serch_var(vars.get_name());
        let serched_var_value = serched_var
          .0
          .ok_or(result::Error::InterpreterError(format!(
            "{} variable not initialized",
            vars.get_name()
          )))?;
        match serched_var_value {
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
              let bin_node = bin
                .get_node_index(0)
                .ok_or(result::Error::InterpreterError(format!("value error")))?;
              match bin_node {
                Syntax::Call(call) => {
                  let serched_function =
                    var
                      .serch_functions(call.get_name())
                      .ok_or(result::Error::InterpreterError(format!(
                        "{} function not found",
                        call.get_name()
                      )))?;
                  self.function_run(&serched_function, call, None)?;
                  return Ok(());
                }
                _ => {
                  return Ok(());
                }
              }
            }
            return Err(result::Error::InterpreterError(format!(
              "{} error dot oprator support",
              var.get_name()
            )));
          }

          _ => {
            return Err(result::Error::InterpreterError(format!(
              "{} is missmatched type",
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

  fn is_node_index(&mut self, vars: &VariableAST, syn: &Syntax) -> Result<(), result::Error> {
    let mut var = VariableAST::new(vars.get_name(), vars.get_is_mutable(), vars.get_is_def());
    var.push_node(self.formula(syn)?);
    return self.push_var(&var);
  }
}
