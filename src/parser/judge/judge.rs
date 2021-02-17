use super::super::parsers::Parsers;
use super::super::ast::ast;
use super::super::super::lexer::token;

static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn judge(&mut self) -> Option<Result<ast::Syntax, String>>  {
    let tokens = self.get_tokens(self.get_index());
    let token = tokens.get_token();
    let len = self.tokens.len();

    if token == TOKEN._let {

    }

    if token == TOKEN._const {

    }

    if token == TOKEN._number {

    }

    if token == TOKEN._variable {

    }

    return Some(Err(format!("syntax error {}", tokens.get_value())));
  }
}