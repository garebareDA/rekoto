use crate::error::result;
use crate::parser::ast;
use crate::parser::ast::ast::{Node, RootAST, Syntax};

#[derive(Debug, Clone)]
struct Variables {
  node: Vec<Vec<ast::ast::VariableAST>>,
}

impl Variables {
  pub fn new() -> Self {
    Self { node: Vec::new() }
  }

  pub fn push_scope(&mut self) {
    self.node.push(Vec::new());
  }

  pub fn push_node(&mut self, node: &ast::ast::VariableAST) {
    let index = self.node.len() - 1;
    self.node[index].push(node.clone());
  }

  pub fn serch(&self, name: &str) -> Option<&Syntax> {
    for i in (0..self.node.len()).rev() {
      for j in (0..self.node[i].len()).rev() {
        let node = &self.node[i][j];
        if name == node.get_name() {
          return node.get_node_index(0);
        }
      }
    }
    return None;
  }
}

#[derive(Debug, Clone)]
struct Functions {
  node: Vec<Vec<ast::ast::FunctionAST>>,
}

impl Functions {
  pub fn new() -> Self {
    Self { node: Vec::new() }
  }

  pub fn push_scope(&mut self) {
    self.node.push(Vec::new());
  }

  pub fn push_node(&mut self, node: &ast::ast::FunctionAST) {
    let index = self.node.len() - 1;
    self.node[index].push(node.clone());
  }
}

#[derive(PartialEq, Debug)]
pub enum InterpreterState {
  Main,
  If,
  IfDone,
  For,
  Fun,
  Call,
}

pub struct Interpreter {
  var: Variables,
  fun: Functions,
  state:Vec<InterpreterState>
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      var: Variables::new(),
      fun: Functions::new(),
      state:Vec::new(),
    }
  }

  pub fn run(&mut self, root: RootAST) -> Result<(), result::Error> {
    self.push_scope();
    self.push_state(InterpreterState::Main);
    for ast in root.get_node().iter() {
      match self.judge(ast).0 {
        Some(judge) => match judge {
          Ok(_) => {
            break;
          }
          Err(e) => {
            return Err(e);
          }
        },
        None => {}
      }
    }
    return Ok(());
  }

  pub fn debug_run(&mut self, root: RootAST) -> Result<Vec<String>, result::Error> {
    self.push_scope();
    self.push_state(InterpreterState::Main);
    let mut log: Vec<String> = Vec::new();
    for ast in root.get_node().iter() {
      match self.judge(ast).1 {
        Some(s) => {
          log.push(s);
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

  pub fn serch_var(&self, name: &str) -> Option<&Syntax> {
    self.var.serch(name)
  }

  pub fn push_fun(&mut self, node: &ast::ast::FunctionAST) {
    self.fun.push_node(node);
  }

  pub fn get_last_state(&self) -> Option<&InterpreterState> {
    self.state.get(self.state.len() - 1)
  }

  pub fn push_state(&mut self, state: InterpreterState) {
    self.state.push(state);
  }

  pub fn pop_state(&mut self) -> InterpreterState {
    self.state.remove(0)
  }
}
