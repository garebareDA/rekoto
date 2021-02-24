use crate::parser::ast::ast::{Syntax, RootAST, Node};

struct Scope {
  node:Vec<Vec<Syntax>>,
}

impl Scope {
  pub fn new() -> Self {
    Self {
      node: Vec::new(),
    }
  }

  pub fn push_scope(&mut self) {
    self.node.push(Vec::new());
  }

  pub fn push_node(&mut self, node:Syntax) {
    let index = self.node.len() - 1;
    self.node[index].push(node);
  }
}

pub struct Interpreter {
  var:Scope,
  fun:Scope,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      var:Scope::new(),
      fun:Scope::new(),
    }
  }

  pub fn run(&self, root:RootAST) -> Result<(), String>{
    for ast in root.get_node().iter() {
      match self.judge(ast) {
        Ok(()) => {}
        Err(e) => {
          return Err(e);
        }
      }
    }

    return Ok(());
  }

  pub fn push_scope(&mut self) {
    self.var.push_scope();
    self.fun.push_scope();
  }

  pub fn push_var(&mut self, node:Syntax) {
    self.var.push_node(node);
  }

  pub fn push_fun(&mut self, node:Syntax) {
    self.push_var(node);
  }
}