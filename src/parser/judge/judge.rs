use super::super::super::lexer::token;
use super::super::ast::ast;
use super::super::parsers::ParseState;
use super::super::parsers::Parsers;

static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn judge(&mut self) -> Option<Result<ast::Syntax, String>> {
    if self.get_index() as usize >= self.get_tokens_len() {
      return None;
    }

    let token = self.get_tokens(self.get_index()).get_token();

    if token == TOKEN._let {
      self.push_state(ParseState::Var);
      self.index_inc();
      let judge = Some(self.variable_def(true));
      self.pop_state();
      return judge;
    }

    if token == TOKEN._const {
      self.push_state(ParseState::Var);
      self.index_inc();
      let judge = Some(self.variable_def(false));
      self.pop_state();
      return judge;
    }

    if token == TOKEN._number {
      return Some(self.number());
    }

    if token == TOKEN._string {
      return Some(self.strings());
    }

    if token == TOKEN._add || token == TOKEN._sub || token == TOKEN._div || token == TOKEN._mul {
      return Some(self.binary());
    }

    if token == TOKEN._equal {
      let value = self
        .get_tokens(self.get_index())
        .get_value()
        .chars()
        .nth(0)
        .unwrap();
      return Some(Ok(ast::Syntax::Bin(ast::BinaryAST::new(value))));
    }

    if token == TOKEN._variable {
      return Some(self.variable(false));
    }

    if token == TOKEN._end {
      return None;
    }

    let value = self.get_tokens(self.get_index()).get_value();
    return Some(Err(format!("syntax error {}", &value)));
  }
}