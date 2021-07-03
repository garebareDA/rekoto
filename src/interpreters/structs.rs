use super::variables;
use crate::parser::ast;

#[derive(Debug, Clone)]
pub struct Structs {
  node: Vec<Vec<ast::ast::StructAST>>,
}

impl Structs {
  pub fn new() -> Self {
    Self { node: Vec::new() }
  }

  pub fn push_node(&mut self, node: &ast::ast::StructAST) {
    let index = self.node.len() - 1;
    self.node[index].push(node.clone());
  }

  pub fn serch(&self, name: &str) -> Option<ast::ast::StructAST> {
    for i in (0..self.node.len()).rev() {
      for j in (0..self.node[i].len()).rev() {
        let node = &self.node[i][j];
        if name == node.get_name() {
          return Some(node.clone());
        }
      }
    }
    return None;
  }
}

impl variables::Scope for Structs {
  fn push_scope(&mut self) {
    self.node.push(Vec::new());
  }

  fn pop_scope(&mut self) {
    self.node.remove(self.node.len() - 1);
  }
}
