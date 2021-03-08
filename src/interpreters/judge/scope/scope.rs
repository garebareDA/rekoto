use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax};

impl Interpreter {
  pub(crate) fn scope(
    &mut self,
    scope: &ast::ScopeAST,
  ) -> (
    Option<Result<Option<Syntax>, result::Error>>,
    Option<String>,
  ) {
    self.push_scope();
    let mut debug:Option<String> = None;
    for ast in scope.get_node().iter() {
      let judge = self.judge(ast);
      debug = judge.1;
      match judge.0 {
        Some(judge) => match judge {
          Ok(ret) => {
            return (Some(Ok(ret)), debug);
          }
          Err(e) => {
            return (Some(Err(e)), None);
          }
        },
        None => {}
      }
    }
    return (None, debug);
  }
}
