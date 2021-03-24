use super::super::ast::{ast, ast::Node};
use super::super::parsers::Parsers;
use crate::error::result;
use crate::lexer::token;
static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn call(&mut self) -> Result<ast::Syntax, result::Error> {
    let name: String;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        name = tokens.get_value().to_string();
      }

      None => {
        return Err(result::Error::SyntaxError(
          "function call error".to_string(),
        ));
      }
    }

    let mut call_ast = ast::CallAST::new(&name);
    self.index_inc();

    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        if tokens.get_token() != TOKEN._paren_left {
          return Err(result::Error::SyntaxError(format!(
            "Error {} not a function",
            tokens.get_value()
          )));
        }
      }

      None => {
        return Err(result::Error::SyntaxError("strings error".to_string()));
      }
    }

    loop {
      self.index_inc();

      match self.get_tokens(self.get_index()) {
        Some(tokens) => {
          if tokens.get_token() == TOKEN._paren_right {
            break;
          }
        }

        None => {
          return Err(result::Error::SyntaxError("strings error".to_string()));
        }
      }

      match self.judge() {
        Some(judge) => {
          let verification_token: i64;
          match self.get_tokens(self.get_index()) {
            Some(tokens) => {
              verification_token = tokens.get_token();
            }
            None => {
              return Err(result::Error::SyntaxError("strings error".to_string()));
            }
          }
          call_ast.push_argment(&judge?);
          if verification_token == TOKEN._paren_right {
            break;
          } else if verification_token == TOKEN._comma {
            continue;
          } else {
            return Err(result::Error::SyntaxError(format!(
              "syntax error  call function {} argments",
              name
            )));
          }
        }
        None => {
          return Err(result::Error::SyntaxError(format!(
            "syntax error call judge"
          )));
        }
      }
    }

    match self.formula_judge() {
      Some(formu) => call_ast.push_node(formu?),
      None => {}
    }

    return Ok(ast::Syntax::Call(call_ast));
  }
}
