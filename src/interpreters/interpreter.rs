use crate::error::result;
use crate::parser::ast;
use crate::parser::ast::ast::{Node, RootAST, Syntax, Types};

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

#[derive(Debug, Clone)]
pub struct Functions {
  node: Vec<Vec<ast::ast::FunctionAST>>,
}

impl Functions {
  pub fn new() -> Self {
    Self { node: Vec::new() }
  }

  pub fn push_scope(&mut self) {
    self.node.push(Vec::new());
  }

  pub fn pop_scope(&mut self) {
    self.node.remove(self.node.len() - 1);
  }

  pub fn push_node(&mut self, node: &ast::ast::FunctionAST) {
    let index = self.node.len() - 1;
    self.node[index].push(node.clone());
  }

  pub fn serch(&self, name: &str) -> Option<ast::ast::FunctionAST> {
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

#[derive(PartialEq, Debug)]
pub enum InterpreterState {
  Main,
  If,
  IfDone,
  For,
  Fun,
  Call,
}

pub struct InterpreterData {
  pub var: Variables,
  pub fun: Functions,
  pub path: String,
  pub name: String,
}

pub struct Interpreter {
  data: Vec<InterpreterData>,
  state: Vec<InterpreterState>,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      data: Vec::new(),
      state: Vec::new(),
    }
  }

  pub fn run(
    &mut self,
    root: RootAST,
    path: impl Into<String>,
    name: impl Into<String>,
  ) -> Result<(), result::Error> {
    self.push_data(path, name);
    self.push_scope();
    self.function_init(&root)?;
    self.push_state(InterpreterState::Call);
    match self.serch_fun("main") {
      Some(main) => {
        for ast in main.get_node().iter() {
          let result = self.judge(ast);
          match result.0 {
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
      }
      None => {
        return Err(result::Error::InterpreterError(
          "not found main fucntion".to_string(),
        ));
      }
    }

    return Ok(());
  }

  pub fn debug_run(&mut self, root: RootAST) -> Result<Vec<String>, result::Error> {
    let mut log: Vec<String> = Vec::new();
    self.push_data("", "");
    self.push_scope();
    self.function_init(&root)?;
    self.push_scope();
    self.push_state(InterpreterState::Call);
    match self.serch_fun("main") {
      Some(main) => {
        for ast in main.get_node().iter() {
          let result = self.judge(ast);

          match result.1 {
            Some(lo) => {
              log.push(lo);
            }

            None => {}
          }

          match result.0 {
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
      }
      None => {
        return Err(result::Error::InterpreterError(
          "not found main fucntion".to_string(),
        ));
      }
    }

    return Ok(log);
  }

  pub fn push_data(&mut self, path: impl Into<String>, name: impl Into<String>) {
    let data = InterpreterData {
      fun: Functions::new(),
      var: Variables::new(),
      path: path.into(),
      name: name.into(),
    };
    self.data.push(data);
  }

  pub fn push_scope(&mut self) {
    self.data[0].var.push_scope();
    self.data[0].fun.push_scope();
  }

  pub fn pop_scope(&mut self) {
    self.data[0].var.pop_scope();
    self.data[0].fun.pop_scope();
  }

  pub fn push_var(&mut self, node: &ast::ast::VariableAST) -> Result<(), result::Error> {
    let a = self.data[0].var.push_node(node);
    return a;
  }

  pub fn serch_var(&self, name: &str) -> (Option<Syntax>, Result<Option<Types>, result::Error>) {
    let mut index = 0;
    for state in self.state.iter() {
      if state == &InterpreterState::Call {
        index += 1;
      }
    }

    let serched = self.data[0].var.serch(name, index);
    match serched {
      Some(var) => match var {
        Syntax::Bool(_) => (Some(var), Ok(Some(Types::Bool))),

        Syntax::Num(_) => (Some(var), Ok(Some(Types::Number))),

        Syntax::Str(_) => (Some(var), Ok(Some(Types::String))),

        _ => {
          return (
            None,
            Err(result::Error::InterpreterError(format!(
              "not found variable {}",
              name
            ))),
          );
        }
      },
      None => {
        return (None, Ok(None));
      }
    }
  }

  pub fn get_path(&self) -> &str {
    &self.data[0].path
  }

  pub fn serch_fun(&self, name: &str) -> Option<ast::ast::FunctionAST> {
    self.data[0].fun.serch(name)
  }

  pub fn push_fun(&mut self, node: &ast::ast::FunctionAST) {
    self.data[0].fun.push_node(node);
  }

  pub fn get_last_state(&self) -> Option<&InterpreterState> {
    self.state.get(self.state.len() - 1)
  }

  pub fn push_state(&mut self, state: InterpreterState) {
    self.state.push(state);
  }

  pub fn pop_state(&mut self) -> InterpreterState {
    return self.state.remove(self.state.len() - 1);
  }
}
