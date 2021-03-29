use crate::error::result;
use crate::parser::ast;
use crate::parser::ast::ast::{Node, RootAST, Syntax, Types};

use super::functions::Functions;
use super::variables::Variables;

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
  path: String,
  name: String,
  state: Vec<InterpreterState>,
}

impl Interpreter {
  pub fn new(path: impl Into<String>, name: impl Into<String>) -> Self {
    Self {
      var: Variables::new(),
      fun: Functions::new(),
      path:path.into(),
      name:name.into(),
      state: Vec::new(),
    }
  }

  pub fn run(
    &mut self,
    root: RootAST,
  ) -> Result<(), result::Error> {
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

  pub fn push_scope(&mut self) {
    self.var.push_scope();
    self.fun.push_scope();
  }

  pub fn pop_scope(&mut self) {
    self.var.pop_scope();
    self.fun.pop_scope();
  }

  pub fn push_var(&mut self, node: &ast::ast::VariableAST) -> Result<(), result::Error> {
    let a = self.var.push_node(node);
    return a;
  }

  pub fn serch_var(&self, name: &str) -> (Option<Syntax>, Result<Option<Types>, result::Error>) {
    let mut index = 0;
    for state in self.state.iter() {
      if state == &InterpreterState::Call {
        index += 1;
      }
    }

    let serched = self.var.serch(name, index);
    match serched {
      Some(var) => match var {
        Syntax::Bool(_) => (Some(var), Ok(Some(Types::Bool))),

        Syntax::Num(_) => (Some(var), Ok(Some(Types::Number))),

        Syntax::Str(_) => (Some(var), Ok(Some(Types::String))),

        Syntax::Var(_) => (Some(var), Ok(None)),

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
    &self.path
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn serch_fun(&self, name: &str) -> Option<ast::ast::FunctionAST> {
    self.fun.serch(name)
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
    return self.state.remove(self.state.len() - 1);
  }
}
