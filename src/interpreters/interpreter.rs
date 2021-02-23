use crate::parser::ast::ast::{Syntax, RootAST, Node};

pub struct Interpreter {
  var:Vec<Vec<Syntax>>,
  fun:Vec<Vec<Syntax>>,
}

impl Interpreter {
  pub fn new() -> Self {
    Self {
      var:Vec::new(),
      fun:Vec::new(),
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
}