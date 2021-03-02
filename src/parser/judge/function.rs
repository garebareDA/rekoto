use super::super::super::lexer::token;
use super::super::ast::{ast, ast::Node, ast::Syntax, ast::Type};
use super::super::parsers::Parsers;
use crate::error::result;

static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn fucntion(&mut self) -> Result<ast::Syntax, result::Error> {
    let mut fn_ast: ast::FunctionAST;
    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          Syntax::Var(var) => {
            if var.get_node_len() < 1 {
              fn_ast = ast::FunctionAST::new(var.get_name());
            } else {
              return Err(result::Error::SyntaxError(
                "fucntion name error possible parser bug".to_string(),
              ));
            }
          }
          _ => {
            return Err(result::Error::SyntaxError(
              "fucntion name error possible parser bug".to_string(),
            ));
          }
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err(result::Error::InterpreterError(
          "fucntion name error possible parser bug".to_string(),
        ));
      }
    }

    self.index_inc();
    let paren_left_token: i64;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        paren_left_token = tokens.get_token();
      }

      None => {
        return Err(result::Error::SyntaxError(
          "out of index fucntion name error possible parser bug".to_string(),
        ));
      }
    }

    if paren_left_token != TOKEN._paren_left {
      return Err(result::Error::SyntaxError("( not enough".to_string()));
    }

    loop {
      self.index_inc();

      let paren_right_token: i64;
      let verification_token: i64;

      match self.get_tokens(self.get_index()) {
        Some(tokens) => {
          paren_right_token = tokens.get_token();
        }

        None => {
          return Err(result::Error::SyntaxError(
            "out of index fucntion param error possible parser bug".to_string(),
          ));
        }
      }

      match self.get_tokens(self.get_index() + 1) {
        Some(tokens) => {
          verification_token = tokens.get_token();
        }

        None => {
          return Err(result::Error::SyntaxError(
            "out of index fucntion param error possible parser bug".to_string(),
          ));
        }
      }

      if paren_right_token == TOKEN._paren_right {
        return Ok(ast::Syntax::Fn(fn_ast));
      }

      match self.judge() {
        Some(judge) => match judge {
          Ok(obj) => match obj {
            Syntax::Var(mut var) => {
              if verification_token != TOKEN._colon {
                return Err(result::Error::SyntaxError(format!(
                  "fucntion {} param type error",
                  fn_ast.get_name()
                )));
              }

              match self.check_types() {
                Ok(types) => {
                  var.set_type(types);
                  fn_ast.push_param(Syntax::Var(var));
                }
                Err(e) => {
                  return Err(e);
                }
              }

              self.index_inc();
              match self.get_tokens(self.get_index()) {
                Some(tokens) => {
                  if tokens.get_token() == TOKEN._paren_right {
                    break;
                  } else if tokens.get_token() == TOKEN._comma {
                    continue;
                  } else {
                    return Err(result::Error::SyntaxError(format!("function ) or , not found {}", fn_ast.get_name())));
                  }
                }

                None => {
                  return Err(result::Error::SyntaxError(format!(
                    "function ) not found {}",
                    fn_ast.get_name()
                  )));
                }
              }
            }

            _ => {
              return Err(result::Error::SyntaxError(format!(
                "fucntion {} param type error",
                fn_ast.get_name()
              )));
            }
          },

          Err(e) => {
            return Err(e);
          }
        },
        None => {
          return Err(result::Error::SyntaxError(format!(
            "functin {} param error",
            fn_ast.get_name()
          )));
        }
      }
    }

    match self.check_types() {
      Ok(types) => {
        fn_ast.set_type(types);
      }
      Err(_) => {
        //返り値がなくても良いため
      }
    }

    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          ast::Syntax::Bin(bin) => {
            return Err(result::Error::SyntaxError(format!(
              "{} syntax error",
              bin.get_bin()
            )))
          }
          _ => fn_ast.push_node(obj),
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err(result::Error::SyntaxError(format!("fucntion scope error")));
      }
    }

    return Ok(ast::Syntax::Fn(fn_ast));
  }
}
