use crate::error::result;
use crate::parser::ast;
use crate::parser::ast::ast::{Node,Syntax};

#[derive(Debug, Clone)]
pub struct Variables {
  node: Vec<Vec<ast::ast::VariableAST>>,
}

impl Variables {
  pub fn new() -> Self {
    Self { node: Vec::new() }
  }

  pub fn push_scope(&mut self) {
    self.node.push(Vec::new());
  }

  pub fn pop_scope(&mut self) {
    self.node.remove(self.node.len() - 1);
  }

  pub fn push_node(&mut self, node: &ast::ast::VariableAST) -> Result<(), result::Error> {
    if node.get_is_def() {
      let index = self.node.len() - 1;
      self.node[index].push(node.clone());
      return Ok(());
    }

    for i in (0..self.node.len()).rev() {
      for j in (0..self.node[i].len()).rev() {
        let nodes = &self.node[i][j];
        if node.get_name() == nodes.get_name() {
          if nodes.get_is_mutable() == true {
            self.node[i][j] = node.clone();
            return Ok(());
          } else {
            return Err(result::Error::InterpreterError(format!(
              "{} is imutable",
              nodes.get_name()
            )));
          }
        }
      }
    }

    return Err(result::Error::InterpreterError(format!(
      "{} is not found variant",
      node.get_name()
    )));
  }

  pub fn serch(&self, name: &str, index: usize) -> Option<Syntax> {
    for i in (index..self.node.len()).rev() {
      for j in (0..self.node[i].len()).rev() {
        let node = &self.node[i][j];
        if name == node.get_name() {
          if node.get_varibale_len() > 0 {
            return Some(Syntax::Var(node.clone()));
          }

          if node.get_function_len() > 0 {
            return Some(Syntax::Var(node.clone()));
          }

          match node.get_node_index(0) {
            Some(node) => {
              return Some(node.clone());
            }

            None => {
              return None;
            }
          }
        }
      }
    }
    return None;
  }
}