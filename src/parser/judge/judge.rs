use super::super::super::lexer::token;
use super::super::ast::ast;
use super::super::parsers::ParseState;
use super::super::parsers::Parsers;
use crate::error::result;

static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn judge(&mut self) -> Option<Result<ast::Syntax, result::Error>> {
    let token: i64;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        token = tokens.get_token();
      }

      None => {
        return None;
      }
    };

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

    if token == TOKEN._false || token == TOKEN._true {
      return Some(self.boolean());
    }

    if token == TOKEN._if {
      self.push_state(ParseState::If);
      let judge = self.ifs();
      self.pop_state();
      return Some(judge);
    }

    if token == TOKEN._else {
      self.push_state(ParseState::If);
      let judge = self.elses();
      self.pop_state();
      return Some(judge);
    }

    if token == TOKEN._elif {
      self.push_state(ParseState::If);
      let judge = self.elif();
      self.pop_state();
      return Some(judge);
    }

    if token == TOKEN._for {
      self.push_state(ParseState::For);
      let judge = self.fors();
      self.pop_state();
      return Some(judge);
    }

    if token == TOKEN._fn {
      self.push_state(ParseState::Function);
      let judge = self.fucntion();
      self.pop_state();
      return Some(judge);
    }

    if token == TOKEN._return {
      return Some(self.returns());
    }

    if token == TOKEN._break {
      return Some(Ok(ast::Syntax::Break));
    }

    if token == TOKEN._add
      || token == TOKEN._sub
      || token == TOKEN._div
      || token == TOKEN._mul
      || token == TOKEN._equ
      || token == TOKEN._sur
      || token == TOKEN._not_equ
      || token == TOKEN._and
      || token == TOKEN._or
      || token == TOKEN._greater
      || token == TOKEN._greater_equ
      || token == TOKEN._less
      || token == TOKEN._less_equ
      || token == TOKEN._dot
    {
      return Some(self.binary());
    }

    if token == TOKEN._equal {
      let value: &str;
      let token: i64;
      match self.get_tokens(self.get_index()) {
        Some(tokens) => {
          value = tokens.get_value();
          token = tokens.get_token();
        }

        None => {
          return Some(Err(result::Error::SyntaxError(
            "syntax error =".to_string(),
          )));
        }
      };
      return Some(Ok(ast::Syntax::Bin(ast::BinaryAST::new(value, token))));
    }

    if token == TOKEN._variable {
      //関数の呼び出しの判定 ( がるか
      match self.get_tokens(self.get_index() + 1) {
        Some(tokens) => {
          let verification_token = tokens.get_token();
          if verification_token == TOKEN._paren_left
            && self.get_last_state() != &ParseState::Function
          {
            self.push_state(ParseState::Call);
            let judge = self.call();
            self.pop_state();
            return Some(judge);
          }
        }

        None => {}
      };

      //変数ならそのまま返す
      return Some(self.variable(false));
    }

    if token == TOKEN._paren_left {
      if self.get_last_state() == &ParseState::Function {
        return None;
      }
    }

    if token == TOKEN._paren_right {
      if self.get_last_state() == &ParseState::Call {
        return None;
      }

      if self.get_last_state() == &ParseState::Function {
        return None;
      }
    }

    if token == TOKEN._braces_left {
      self.push_state(ParseState::Scope);
      return Some(self.scope());
    }

    if token == TOKEN._braces_right {
      if self.get_last_state() != &ParseState::Scope {
        return Some(Err(result::Error::SyntaxError(
          "Scope { is not".to_string(),
        )));
      }
      self.pop_state();
      return None;
    }

    if token == TOKEN._comma {
      if self.get_last_state() == &ParseState::Call {
        return None;
      }

      if self.get_last_state() == &ParseState::Function {
        return None;
      }
    }

    if token == TOKEN._end {
      return None;
    }

    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        return Some(Err(result::Error::SyntaxError(format!(
          "syntax error {}",
          tokens.get_value()
        ))));
      }

      None => {
        return Some(Err(result::Error::SyntaxError(
          "syntax error possible parser bug".to_string(),
        )));
      }
    };
  }
}
