use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast::{Node, Syntax, VariableAST};

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
    match self.formula(syn) {
      Ok(inner) => {
        let mut var = VariableAST::new(vars.get_name(), vars.get_is_mutable(), vars.get_is_def());
        var.push_node(inner);
        return self.push_var(&var);
      }

      Err(e) => {
        return Err(e);
      }
    }
  }
}
