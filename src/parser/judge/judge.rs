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
      let judge = self.variable_def(true);
      self.pop_state();
      return Some(judge);
    }

    if token == TOKEN._const {
      self.push_state(ParseState::Var);
      self.index_inc();
      let judge = self.variable_def(false);
      self.pop_state();
      return Some(judge);
    }

    if token == TOKEN._number {
      return Some(self.number());
    }

    if token == TOKEN._string {
      return Some(self.strings());
    }

    if token == TOKEN._if {
      self.push_state(ParseState::If);
      let judge = self.ifs();
      self.pop_state();
      return Some(judge);
    }

    if token == TOKEN._add
      || token == TOKEN._sub
      || token == TOKEN._div
      || token == TOKEN._mul
      || token == TOKEN._equ
      || token == TOKEN._not_equ
      || token == TOKEN._and
      || token == TOKEN._or
      || token == TOKEN._greater
      || token == TOKEN._greater_equ
      || token == TOKEN._less
      || token == TOKEN._less_equ
    {
      return Some(self.binary());
    }

    if token == TOKEN._equal {
      let value = self.get_tokens(self.get_index()).get_value();
      return Some(Ok(ast::Syntax::Bin(ast::BinaryAST::new(value))));
    }

    if token == TOKEN._variable {
      //関数の呼び出しの判定
      let verification_token = self.get_tokens(self.get_index() + 1).get_token();
      if verification_token == TOKEN._paren_left {
        self.push_state(ParseState::Call);
        let judge = self.call();
        self.pop_state();
        return Some(judge);
      }

      //変数ならそのまま返す
      return Some(self.variable(false));
    }

    if token == TOKEN._paren_right {
      if self.get_last_state() == &ParseState::Call {
        return None;
      }
    }

    if token == TOKEN._braces_left {
      self.push_state(ParseState::Scope);
      return Some(self.scope());
    }

    if token == TOKEN._braces_right {
      if self.get_last_state() != &ParseState::Scope {
        return Some(Err("Scope is not".to_string()));
      }
      self.pop_state();
      return None;
    }

    if token == TOKEN._comma {
      if self.get_last_state() == &ParseState::Call {
        return None;
      }
    }

    if token == TOKEN._end {
      return None;
    }

    let value = self.get_tokens(self.get_index()).get_value();
    return Some(Err(format!("syntax error {}", &value)));
  }
}
