use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast::{Node, Syntax, VariableAST};

impl Interpreter {
  pub(crate) fn variable(&mut self, var: &VariableAST) -> Result<(), result::Error> {
    if var.get_node_len() == 0 {
      self.push_var(var);
      return Ok(());
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
          Syntax::Var(var2) => {
            //変数の検索
           return self.is_node_index(var2, vars, var);
          }

          Syntax::Str(strs) => {
            return self.is_node_index(strs, vars, var);
          }

          Syntax::Bool(bools) => {
            return self.is_node_index(bools, vars, var);
          }

          Syntax::Call(call) => {
            return self.is_node_index(call, vars, var);
          }

          Syntax::Num(num) => {
            return self.is_node_index(num, vars, var);
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

  fn is_node_index<T: Node>(&mut self, t: &T, vars: &VariableAST, syn: &Syntax) -> Result<(), result::Error> {
    let is = 0 < t.get_node_len();
    if !is {
      self.push_var(vars);
      return Ok(());
    } else {
      match self.formula(syn) {
        Ok(inner) => {
          let mut var = VariableAST::new(vars.get_name(), vars.get_is_mutable(), vars.get_is_def());
          var.push_node(inner);
          return Ok(());
        }

        Err(e) => {
          return Err(e);
        }
      }
    }
  }
}
