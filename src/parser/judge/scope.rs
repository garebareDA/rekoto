use super::super::ast::{ast, ast::Node, ast::Syntax};
use super::super::parsers::{ParseState, Parsers};
use crate::error::result;

impl Parsers {
  pub(crate) fn scope(&mut self) -> Result<Syntax, result::Error> {
    let mut scope_ast = ast::ScopeAST::new();

    loop {
      if self.get_last_state() != &ParseState::Scope {
        break;
      }

      if self.get_tokens_len() <= self.get_index() as usize {
        return Err(result::Error::SyntaxError(format!("invalid scope")));
      }

      self.index_inc();
      match self.judge() {
        Some(judge) => scope_ast.push_node(judge?),
        None => {}
      }
    }

    return Ok(Syntax::Scope(scope_ast));
  }
}
