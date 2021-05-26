use super::super::ast::{ast, ast::Node};
use super::super::parsers::Parsers;
use crate::error::result;
use crate::lexer::token;
static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn call(&mut self) -> Result<ast::Syntax, result::Error> {
    let name = self
      .get_tokens(self.get_index())
      .ok_or(result::Error::SyntaxError(
        "function call error".to_string(),
      ))?
      .get_value()
      .to_string();

    let mut call_ast = ast::CallAST::new(&name);
    self.index_inc();

    let tokens = self
      .get_tokens(self.get_index())
      .ok_or(result::Error::SyntaxError("strings error".to_string()))?;

    if tokens.get_token() != TOKEN._paren_left {
      return Err(result::Error::SyntaxError(format!(
        "Error {} not a function",
        tokens.get_value()
      )));
    }

    loop {
      self.index_inc();
      let tokens = self
        .get_tokens(self.get_index())
        .ok_or(result::Error::SyntaxError("strings error".to_string()))?;
      if tokens.get_token() == TOKEN._paren_right {
        break;
      }

      let judge = self.judge().ok_or(result::Error::SyntaxError(format!(
        "syntax error call judge"
      )))?;

      let verification_token = self
        .get_tokens(self.get_index())
        .ok_or(result::Error::SyntaxError("strings error".to_string()))?
        .get_token();

      call_ast.push_argment(&judge?);
      if verification_token == TOKEN._paren_right {
        break;
      } else if verification_token == TOKEN._comma {
        continue;
      } else {
        return Err(result::Error::SyntaxError(format!(
          "syntax error  call function {} argments",
          name,
        )));
      }
    }

    match self.formula_judge() {
      Some(formu) => call_ast.push_node(formu?),
      None => {}
    }

    return Ok(ast::Syntax::Call(call_ast));
  }
}
