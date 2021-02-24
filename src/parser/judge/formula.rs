use super::super::super::lexer::token;
use super::super::ast::{ast, ast::Node};
use super::super::parsers::{ParseState, Parsers};

static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn number(&mut self) -> Result<ast::Syntax, String> {
    let num: i64;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        num = tokens.get_value().parse().unwrap();
      }

      None => {
        return Err("number error".to_string());
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

  pub(crate) fn binary(&mut self) -> Result<ast::Syntax, String> {
    let value: &str;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        value = tokens.get_value();
      }

      None => {
        return Err("binary error".to_string());
      }
    }
    let mut ch_ast = ast::BinaryAST::new(value);

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

  pub(crate) fn strings(&mut self) -> Result<ast::Syntax, String> {
    let strs: &str;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        strs = tokens.get_value();
      }

      None => {
        return Err("strings error".to_string());
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

  pub(crate) fn formula_judge(&mut self) -> Option<Result<ast::Syntax, String>> {
    //judge()で判定するとインクリメントされるため
    match self.get_tokens(self.get_index() + 1) {
      Some(tokens) => {
        if (self.get_last_state() == &ParseState::If
          || self.get_last_state() == &ParseState::For
          || self.get_last_state() == &ParseState::Function)
          && tokens.get_token() == TOKEN._braces_left
        {
          return None;
        }

        if self.get_last_state() == &ParseState::Function
          && tokens.get_token() == TOKEN._paren_left
        {
          return None;
        }

        if self.get_last_state() == &ParseState::Function
          && tokens.get_token() == TOKEN._colon {
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

          ast::Syntax::Var(var) => {
            if var.get_is_def() {
              return Some(Err(format!("syntax error let")));
            }
            return Some(Ok(ast::Syntax::Var(var)));
          }

          ast::Syntax::Call(call) => {
            return Some(Ok(ast::Syntax::Call(call)));
          }

          ast::Syntax::Scope(_) => {
            return Some(Err(format!("syntax error scope")));
          }

          _ => {
            return Some(Err(format!("syntax error scope")));
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
}
