use super::super::super::lexer::token;
use super::super::ast::{ast, ast::Node, ast::Syntax};
use super::super::parsers::Parsers;

static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn fucntion(&mut self) -> Result<ast::Syntax, String> {
    let mut fn_ast: ast::FunctionAST;
    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          Syntax::Var(var) => {
            if var.get_node_len() < 1 {
              fn_ast = ast::FunctionAST::new(var.get_name());
            } else {
              return Err("fucntion name error".to_string());
            }
          }
          _ => {
            return Err("fucntion name error".to_string());
          }
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err("fucntion name error".to_string());
      }
    }

    self.index_inc();
    let paren_left_token:i64;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        paren_left_token = tokens.get_token();
      }

      None =>{
        return Err("fucntion name error".to_string());
      }
    }

    if paren_left_token != TOKEN._paren_left {
      return Err("( not enough".to_string());
    }

    loop {
      self.index_inc();

      let paren_right_token:i64;
      let verification_token:i64;

      match self.get_tokens(self.get_index()) {
        Some(tokens) => {
          paren_right_token = tokens.get_token();
        }

        None =>{
          return Err("fucntion param error".to_string());
        }
      }

      match self.get_tokens(self.get_index() + 1) {
        Some(tokens) => {
          verification_token = tokens.get_token();
        }

        None =>{
          return Err("fucntion param error".to_string());
        }
      }

      if paren_right_token == TOKEN._paren_right {
        return Ok(ast::Syntax::Fn(fn_ast));
      }

      match self.judge() {
        Some(judge) => match judge {
          Ok(obj) => {
            fn_ast.push_param(&obj);
            if verification_token == TOKEN._paren_right {
              break;
            } else if verification_token == TOKEN._comma {
              continue;
            } else {
              return Err(format!("Syntax error {}", fn_ast.get_name()));
            }
          }

          Err(e) => {
            return Err(e);
          }
        },
        None => {
          return Err(format!("functin param error"));
        }
      }
    }
    self.index_inc();

    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          ast::Syntax::Bin(bin) => return Err(format!("{} syntax error", bin.get_bin())),
          _ => fn_ast.push_node(obj),
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err(format!("fucntion scope error"));
      }
    }

    return Ok(ast::Syntax::Fn(fn_ast));
  }
}
