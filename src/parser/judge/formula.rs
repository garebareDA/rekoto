use super::super::super::lexer::token;
use super::super::ast::{ast, ast::Node};
use super::super::parsers::{ParseState, Parsers};
use crate::error::result;

static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn number(&mut self) -> Result<ast::Syntax, result::Error> {
    let num: i64;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        num = tokens.get_value().parse().unwrap();
      }

      None => {
        return Err(result::Error::SyntaxError(
          "out of index number error possible parser bug".to_string(),
        ));
      }
    }

    let mut num_ast = ast::NumberAST::new(num);

    match self.formula_judge() {
      Some(formu) => match formu {
        Ok(objf) => {
          num_ast.push_node(objf);
        }
        Err(e) => {
          return Err(e);
        }
      },
      None => {}
    }

    return Ok(ast::Syntax::Num(num_ast));
  }

  pub(crate) fn binary(&mut self) -> Result<ast::Syntax, result::Error> {
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        let mut value = tokens.get_value();
        let mut token = tokens.get_token();

        if token == TOKEN._less
          || token == TOKEN._greater
          || token == TOKEN._nega
          || token == TOKEN._equal
        {
          match self.get_tokens(self.get_index() + 1) {
            Some(tokens) => {
              let tokens_is_eq = tokens.get_token();
              if tokens_is_eq == TOKEN._equal {
                if token == TOKEN._less {
                  value = ">=";
                  token = TOKEN._less_equ;
                } else if token == TOKEN._greater {
                  value = "<=";
                  token = TOKEN._greater_equ;
                } else if token == TOKEN._nega {
                  value = "!=";
                  token = TOKEN._not_equ;
                } else if token == TOKEN._equal {
                  value = "==";
                  token = TOKEN._equ;
                } else {
                  return Err(result::Error::SyntaxError(format!(
                    "oprator error {}{}",
                    value,
                    tokens.get_value()
                  )));
                }

                self.index_inc();
              }
            }
            None => {}
          }
        }

        if token == TOKEN._add {
          match self.formula_same_check(TOKEN._add, value) {
            Ok(()) => {
              self.index_inc();
              value = "++";
              token = TOKEN._inc;
            }

            Err(_) => {}
          }
        }

        if token == TOKEN._sub {
          match self.formula_same_check(TOKEN._sub, value) {
            Ok(()) => {
              self.index_inc();
              value = "--";
              token = TOKEN._dec;
            }

            Err(_) => {}
          }
        }

        if token == TOKEN._pipe {
          match self.formula_same_check(TOKEN._pipe, value) {
            Ok(()) => {
              self.index_inc();
              value = "||";
              token = TOKEN._or;
            }

            Err(e) => {
              return Err(e);
            }
          }
        }

        if token == TOKEN._amp {
          match self.formula_same_check(TOKEN._amp, value) {
            Ok(()) => {
              self.index_inc();
              value = "&&";
              token = TOKEN._and;
            }

            Err(e) => {
              return Err(e);
            }
          }
        }

        if token == TOKEN._equal {
          return Ok(ast::Syntax::Bin(ast::BinaryAST::new(value, token)));
        }

        let mut ch_ast = ast::BinaryAST::new(value, token);
        match self.formula_judge() {
          Some(formu) => match formu {
            Ok(objf) => {
              ch_ast.push_node(objf);
            }
            Err(e) => {
              return Err(e);
            }
          },
          None => {}
        }
        return Ok(ast::Syntax::Bin(ch_ast));
      }

      None => {
        return Err(result::Error::SyntaxError(
          "out of index binary error possible parser bug".to_string(),
        ));
      }
    }
  }

  pub(crate) fn strings(&mut self) -> Result<ast::Syntax, result::Error> {
    let strs: &str;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        strs = tokens.get_value();
      }

      None => {
        return Err(result::Error::SyntaxError(
          "out of index strings error possible parser bug".to_string(),
        ));
      }
    }

    let mut str_ast = ast::StringAST::new(strs);

    match self.formula_judge() {
      Some(formu) => match formu {
        Ok(obj) => {
          str_ast.push_node(obj);
        }
        Err(e) => {
          return Err(e);
        }
      },
      None => {}
    }
    return Ok(ast::Syntax::Str(str_ast));
  }

  pub(crate) fn boolean(&mut self) -> Result<ast::Syntax, result::Error> {
    let mut bools: ast::BoolAST;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        let token = tokens.get_token();
        if token == TOKEN._false {
          bools = ast::BoolAST::new(false);
        } else if token == TOKEN._true {
          bools = ast::BoolAST::new(true);
        } else {
          return Err(result::Error::SyntaxError(format!(
            "not boolean {} possible parser bug",
            tokens.get_value()
          )));
        }
      }

      None => {
        return Err(result::Error::SyntaxError(
          "out of index error bool error possible parser bug".to_string(),
        ));
      }
    }

    match self.formula_judge() {
      Some(formu) => match formu {
        Ok(obj) => {
          bools.push_node(obj);
        }
        Err(e) => {
          return Err(e);
        }
      },
      None => {}
    }
    return Ok(ast::Syntax::Bool(bools));
  }

  pub(crate) fn formula_judge(&mut self) -> Option<Result<ast::Syntax, result::Error>> {
    //judge()で判定するとインクリメントされるため
    match self.get_tokens(self.get_index() + 1) {
      Some(tokens) => {
        let state = self.get_last_state();
        let token = tokens.get_token();
        if (state == &ParseState::Var || state == &ParseState::Call) && token == TOKEN._braces_right
        {
          return None;
        }

        if (state == &ParseState::If || state == &ParseState::For || state == &ParseState::Function)
          && (token == TOKEN._braces_left || token == TOKEN._braces_right)
        {
          return None;
        }

        if state == &ParseState::Function && token == TOKEN._paren_left {
          return None;
        }

        if state == &ParseState::Function && token == TOKEN._colon {
          return None;
        }
      }

      None => {
        return None;
      }
    }

    self.index_inc();
    let judge = self.judge();
    match judge {
      Some(obj) => match obj {
        Ok(syn) => match syn {
          ast::Syntax::Bin(bin) => {
            return Some(Ok(ast::Syntax::Bin(bin)));
          }

          ast::Syntax::Num(num) => {
            return Some(Ok(ast::Syntax::Num(num)));
          }

          ast::Syntax::Str(strs) => {
            return Some(Ok(ast::Syntax::Str(strs)));
          }

          ast::Syntax::Bool(bools) => {
            return Some(Ok(ast::Syntax::Bool(bools)));
          }

          ast::Syntax::Var(var) => {
            if var.get_is_def() {
              return Some(Err(result::Error::SyntaxError(format!("syntax error let"))));
            }
            return Some(Ok(ast::Syntax::Var(var)));
          }

          ast::Syntax::Call(call) => {
            return Some(Ok(ast::Syntax::Call(call)));
          }

          ast::Syntax::Scope(_) => {
            return Some(Err(result::Error::SyntaxError(format!(
              "syntax error scope"
            ))));
          }

          _ => {
            return Some(Err(result::Error::SyntaxError(format!(
              "syntax error scope"
            ))));
          }
        },
        Err(e) => {
          return Some(Err(e));
        }
      },
      None => {
        return None;
      }
    }
  }

  pub(crate) fn formula_same_check(&self, check: i64, value: &str) -> Result<(), result::Error> {
    match self.get_tokens(self.get_index() + 1) {
      Some(tokens) => {
        let tokens_is = tokens.get_token();
        if tokens_is == check {
          return Ok(());
        } else {
          return Err(result::Error::SyntaxError(format!(
            "oprator error {}{}",
            value,
            tokens.get_value()
          )));
        }
      }
      None => {
        return Err(result::Error::SyntaxError(format!(
          "oprator error {}",
          value,
        )));
      }
    }
  }
}
