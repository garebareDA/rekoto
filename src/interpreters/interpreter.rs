use crate::error::result;
use crate::parser::ast;
use crate::parser::ast::ast::{Node, RootAST, Syntax, Types};

use super::variables::Scope;
use super::variables::Variables;
use super::functions::Functions;
use super::structs::Structs;

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
  structs: Structs,
  path: String,
  name: String,
  state: Vec<InterpreterState>,
}

impl Interpreter {
  pub fn new(path: impl Into<String>, name: impl Into<String>) -> Self {
    Self {
      var: Variables::new(),
      fun: Functions::new(),
      structs: Structs::new(),
      path: path.into(),
      name: name.into(),
      state: Vec::new(),
    }
  }

  pub fn run(&mut self, root: RootAST) -> Result<(), result::Error> {
    self.push_scope();
    self.interpreter_init(&root)?;
    self.push_state(InterpreterState::Main);
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
    self.interpreter_init(&root)?;
    self.push_scope();
    self.push_state(InterpreterState::Main);
    let main = self
      .serch_fun("main")
      .ok_or(result::Error::InterpreterError(
        "not found main fucntion".to_string(),
      ))?;

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

    return Ok(log);
  }

  fn interpreter_init(&mut self, root: &ast::ast::RootAST) -> Result<(), result::Error> {
    for ast in root.get_node().iter() {
      match ast {
        Syntax::Fn(fun) => {
          self.push_fun(fun);
        }

        Syntax::Var(var) => {
          //下の階層にあれば計算してvarにpush
          //なければそのままvar_push
          self.variable(var)?;
        }

        Syntax::Struct(structs) => {
          self.push_struct(structs);
        },

        Syntax::Import(import) => {
          let inner = import
            .get_node_index(0)
            .ok_or(result::Error::InterpreterError(format!("import error")))?;

          match inner {
            Syntax::Str(strs) => {
              self.push_var(&self.import(strs.get_str())?)?;
            }

            _ => {
              return Err(result::Error::InterpreterError(
                "please specify import as a string ".to_string(),
              ));
            }
          }
        }

        _ => {
          return Err(result::Error::InterpreterError(
            "the syntax is not written inside the function".to_string(),
          ));
        }
      }
    }
    return Ok(());
  }

  pub fn push_scope(&mut self) {
    self.var.push_scope();
    self.fun.push_scope();
    self.structs.push_scope();
  }

  pub fn pop_scope(&mut self) {
    self.var.pop_scope();
    self.fun.pop_scope();
    self.structs.pop_scope();
  }

  pub fn push_var(&mut self, node: &ast::ast::VariableAST) -> Result<(), result::Error> {
    self.var.push_node(node)
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

  pub fn serch_struct(&self, name: &str) -> Option<ast::ast::StructAST> {
    self.structs.serch(name)
  }

  pub fn push_struct(&mut self, node: &ast::ast::StructAST) {
    self.structs.push_node(node);
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
