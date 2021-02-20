use super::super::parsers::{Parsers, ParseState};
use super::super::ast::ast::Syntax;
use super::super::ast::ast;

impl Parsers {
  pub(crate) fn scope(&mut self) -> Result<Syntax, String> {
    let mut scope_ast = ast::ScopeAST::new();

    loop {
      self.index_inc();
      if self.get_last_state() != &ParseState::Scope {
        break;
      }

      if self.get_tokens_len() <= self.get_index() as usize {
        return Err(format!("Invalid scope"));
      }

      match self.judge() {
        Some(judge) => {
          match judge {
            Ok(obj) => {
              scope_ast.push_scope(&obj);
            }

            Err(e) => {
              return Err(e);
            }
          }
        }
        None => {}
      }
    }

    self.index_inc();
    return Ok(Syntax::Scope(scope_ast));
  }
}