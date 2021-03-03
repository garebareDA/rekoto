use crate::parser::ast::ast::{Syntax, RootAST, Node};
use crate::parser::ast;
use crate::error::result;

#[derive(Debug, Clone)]
struct Variables {
  node:Vec<Vec<ast::ast::VariableAST>>,
}

impl Variables {
  pub fn new() -> Self {
    Self {
      node: Vec::new(),
    }
  }

  pub fn push_scope(&mut self) {
    self.node.push(Vec::new());
  }

  pub fn push_node(&mut self, node: &ast::ast::VariableAST) {
    let index = self.node.len() - 1;
    self.node[index].push(node.clone());
  }

  pub fn serch(&self, name:&str) -> Option<&Syntax> {
    for i in (0..self.node.len()).rev() {
      for j in (0..self.node[i].len()).rev() {
        let node = &self.node[i][j];
        if name ==  node.get_name() {
          return node.get_node_index(0);
        }
      }
    }
    return None;
  }
}

#[derive(Debug, Clone)]
struct Functions {
  node:Vec<Vec<ast::ast::FunctionAST>>,
}

impl Functions {
  pub fn new() -> Self {
    Self {
      node: Vec::new(),
    }
  }

  pub fn push_scope(&mut self) {
    self.node.push(Vec::new());
  }

  pub fn push_node(&mut self, node: &ast::ast::FunctionAST) {
    let index = self.node.len() - 1;
    self.node[index].push(node.clone());
  }
}


pub struct Interpreter {
  var:Variables,
  fun:Functions,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      var:Variables::new(),
      fun:Functions::new(),
    }
  }

  pub fn run(&mut self, root:RootAST) -> Result<(), result::Error>{
    self.push_scope();
    for ast in root.get_node().iter() {
      match self.judge(ast) {
        Some(judge) => {
          match judge {
            Ok(_) => {}
            Err(e) => {
              return Err(e);
            }
          }
        }
        None => {}
      }
    }

    return Ok(());
  }

  pub fn debug_run(&mut self, root:RootAST) -> Result<Vec<String>, result::Error>{
    self.push_scope();
    let mut log:Vec<String> = Vec::new();
    for ast in root.get_node().iter() {
      match self.judge(ast) {
        Some(judge) => {
          match judge {
            Ok(s) => {
              log.push(s);
            }
            Err(e) => {
              return Err(e);
            }
          }
        }
        None => {}
      }
    }

    return Ok(log);
  }

  pub fn push_scope(&mut self) {
    self.var.push_scope();
    self.fun.push_scope();
  }

  pub fn push_var(&mut self, node: &ast::ast::VariableAST) {
    self.var.push_node(node);
  }

  pub fn serch_var(&self, name:&str) -> Option<&Syntax>{
    self.var.serch(name)
  }

  pub fn push_fun(&mut self, node: &ast::ast::FunctionAST) {
    self.fun.push_node(node);
  }
}