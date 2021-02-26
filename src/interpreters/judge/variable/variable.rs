use super::super::super::interpreter::Interpreter;
use crate::parser::ast::ast::{VariableAST, Node, Syntax};

impl Interpreter{
  pub(crate) fn variable(&mut self, var: &VariableAST) -> Result<(), String> {
    if var.get_node_len() == 0 {
      self.push_var(var);
      return Ok(());
    }

    if var.get_node_len() == 1 {
      return self.substitution_one_node(var);
    }

    return Ok(());
  }

  fn substitution_one_node(&mut self, vars: &VariableAST) -> Result<(), String> {
    match vars.get_node_index(0) {
      Some(var) => {
        match var {
          Syntax::Var(var2) => {
            //変数の検索
            self.push_var(vars2);
            return Ok(());
          }

          Syntax::Str(_) => {
            self.push_var(vars);
            return Ok(());
          }

          Syntax::Bool(_) => {
            self.push_var(vars);
            return Ok(());
          }

          Syntax::Call(_) => {
            self.push_var(vars);
            return Ok(());
          }

          Syntax::Num(_) => {
            self.push_var(vars);
            return Ok(());
          }

          _ => {
            //error
            return Err("Cannot assign to variable".to_string());
          }
        }
      }

      None => {
        return Err("Cannot assign to variable".to_string());
      }
    }
  }
}