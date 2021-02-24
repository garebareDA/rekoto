use crate::parser::ast::ast::{Syntax, RootAST, Node};
use std::io::{self, Write};

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
  out:String,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      var:Scope::new(),
      fun:Scope::new(),
      out:"".to_string(),
    }
  }

  pub fn run(&mut self, root:RootAST) -> Result<(), String>{
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

  pub fn set_out(&mut self, message:impl Into<String>) {
    self.out = message.into();
  }

  pub fn get_out(&self) -> &str {
    &self.out
  }

  pub fn print_out(&self) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(self.get_out().as_bytes())?;
    return Ok(());
  }
}