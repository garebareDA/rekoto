use super::super::ast::{ast, ast::Node};
use super::super::parsers::Parsers;
use crate::lexer::token;
static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn call(&mut self) -> Result<ast::Syntax, String> {
    let name = self.get_tokens(self.get_index()).get_value().to_string();
    let mut call_ast = ast::CallAST::new(&name);

    self.index_inc();
    let paren_left_token = self.get_tokens(self.get_index()).get_token();
    if paren_left_token != TOKEN._paren_left {
      return Err(format!("Error not a function"));
    }

    loop {
      self.index_inc();
      let paren_right_token = self.get_tokens(self.get_index()).get_token();
      let verification_token = self.get_tokens(self.get_index() + 1).get_token();

      if paren_right_token == TOKEN._paren_right {
        return Ok(ast::Syntax::Call(call_ast));
      }

      match self.judge() {
        Some(judge) => match judge {
          Ok(obj) => {
            call_ast.push_argment(&obj);
            if verification_token == TOKEN._paren_right {
              break;
            } else if verification_token == TOKEN._comma {
              continue;
            } else {
              return Err(format!("Syntax error {}", name));
            }
          }

          Err(e) => {
            return Err(e);
          }
        },
        None => {
          return Err(format!("syntax error call judge"));
        }
      }
    }

    match self.formula_judge() {
      Some(formu) => match formu {
        Ok(obj) => {
          call_ast.push_node(obj);
        }
        Err(e) => {
          return Err(e);
        }
      },
      None => {}
    }

    return Ok(ast::Syntax::Call(call_ast));
  }
}
