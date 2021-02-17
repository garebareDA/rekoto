use super::super::parsers::Parsers;
use super::super::ast::ast;
use super::super::super::lexer::token;

static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn judge(&mut self) -> Option<Result<ast::Syntax, String>>  {
    let token = self.get_tokens(self.get_index()).get_token();
    let len = self.tokens.len();

    if token == TOKEN._let {
      self.index_inc();
      match self.variable_def(true) {
        Ok(syntax) => {
          return Some(Ok(syntax));
        }

        Err(s) => {
          return Some(Err(s));
        }
      }
    }

    if token == TOKEN._const {
      self.index_inc();
      match self.variable_def(false) {
        Ok(syntax) => {
          return Some(Ok(syntax));
        }

        Err(s) => {
          return Some(Err(s));
        }
      }
    }

    if token == TOKEN._number {
      
    }

    if token == TOKEN._variable {
      let value = self.get_tokens(self.get_index()).get_value();
      self.variable(value);
    }

    if token == TOKEN._equal {

    }

    let value = self.get_tokens(self.get_index()).get_value();
    return Some(Err(format!("syntax error {}", &value)));
  }
}