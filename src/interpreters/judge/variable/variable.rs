use super::super::super::interpreter::Interpreter;
use crate::parser::ast::ast::{VariableAST, Node, Syntax};

impl Interpreter{
  pub(crate) fn variable(&mut self, var: &VariableAST) {
    if var.get_is_def() == false {
      //使われている変数
    }

    if var.get_node_len() < 2 {
      self.push_var(var);
    }
  }
}