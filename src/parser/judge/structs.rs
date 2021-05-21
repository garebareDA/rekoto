use super::super::super::lexer::token;
use super::super::ast::{ast, ast::Node, ast::Syntax};
use super::super::parsers::ParseState;
use super::super::parsers::Parsers;
use crate::error::result;

static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn structs(&mut self) -> Result<ast::Syntax, result::Error> {
    let name: String;
    self.index_inc();
    match self.judge() {
      Some(judge) => match judge? {
        Syntax::Var(var) => {
          if var.get_node_len() < 1 {
            name = var.get_name().to_string();
          } else {
            return Err(result::Error::SyntaxError(
              "struct name error possible parser bug".to_string(),
            ));
          }
        }
        _ => {
          return Err(result::Error::SyntaxError(
            "struct name error possible parser bug".to_string(),
          ));
        }
      },
      None => {
        return Err(result::Error::SyntaxError(
          "struct name error possible parser bug".to_string(),
        ));
      }
    }

    self.index_inc();
    match self.judge() {
      Some(judge) => match judge? {
        Syntax::Struct(mut st) => {
          st.set_name(name);
          return Ok(Syntax::Struct(st));
        }

        _ => {}
      },
      None => {}
    }

    return Err(result::Error::SyntaxError("sytax error struct".to_string()));
  }

  pub(crate) fn member(&mut self) -> Result<ast::Syntax, result::Error> {
    self.index_inc();
    let mut structs_ast = ast::StructAST::new("");
    self.push_state(ParseState::Member);
    loop {
      let name;
      let member_types;

      match self.judge() {
        Some(judge) => match judge? {
          Syntax::Var(var) => {
            if var.get_node_len() < 1 {
              name = var.get_name().to_string();
            } else {
              return Err(result::Error::SyntaxError(
                "member name error possible parser bug".to_string(),
              ));
            }
          }

          _ => {
            return Err(result::Error::SyntaxError(
              "member syntax error".to_string(),
            ));
          }
        },

        None => {
          if self.get_last_state() != &ParseState::Member {
            break;
          }

          return Err(result::Error::SyntaxError(
            "member syntax error".to_string(),
          ));
        }
      }

      match self.check_types() {
        Ok(types) => {
          member_types = types;
          self.index_inc();
        }
        Err(e) => {
          return Err(e);
        }
      }

      match self.judge() {
        Some(_) => {
          return Err(result::Error::SyntaxError(
            "member syntax commma error".to_string(),
          ));
        }
        None => {
          self.index_inc();
        }
      }

      let member_ast = ast::MemberAST::new(member_types, name);
      structs_ast.push_member(&member_ast);
    }

    return Ok(Syntax::Struct(structs_ast));
  }

  pub(crate) fn instance(&mut self) -> Result<ast::Syntax, result::Error> {
    let name: String;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        name = tokens.get_value().to_string();
      }

      None => {
        return Err(result::Error::SyntaxError(
          "struct instance error".to_string(),
        ));
      }
    }

    self.index_inc();
    match self.judge() {
      Some(judge) => match judge? {
        Syntax::Struct(mut structs) => {
          structs.set_name(name);
          return Ok(ast::Syntax::Struct(structs));
        }

        _ => {}
      },
      None => {}
    }

    return Err(result::Error::SyntaxError(
      "struct instance error".to_string(),
    ));
  }

  pub(crate) fn instance_member(&mut self) -> Result<ast::Syntax, result::Error> {
    self.index_inc();
    let mut structs_ast = ast::StructAST::new("");

    loop {
      let name;
      let member;
      match self.judge() {
        Some(judge) => match judge? {
          Syntax::Var(var) => {
            if var.get_node_len() < 1 {
              name = var.get_name().to_string();
            } else {
              return Err(result::Error::SyntaxError(
                "instance member name error possible parser bug".to_string(),
              ));
            }
          }

          _ => {
            return Err(result::Error::SyntaxError(
              "instance member syntax error".to_string(),
            ));
          }
        },

        None => {
          if self.get_last_state() != &ParseState::New {
            break;
          }

          return Err(result::Error::SyntaxError(
            "instance member syntax error".to_string(),
          ));
        }
      }

      match self.get_tokens(self.get_index() + 1) {
        Some(tokens) => {
          if tokens.get_token() != TOKEN._colon {
            return Err(result::Error::SyntaxError(
              "instance member colon noting error".to_string(),
            ));
          }
        }

        None => {
          return Err(result::Error::SyntaxError(
            "instance member colon noting error".to_string(),
          ))
        }
      }

      self.index_inc();

      match self.judge() {
        Some(judge) => {
          member = judge?;
        }
        None => {
          return Err(result::Error::SyntaxError(
            "instance member error".to_string(),
          ))
        }
      }

      match self.judge() {
        Some(_) => {
          return Err(result::Error::SyntaxError(
            "instance member comma nothing".to_string(),
          ))
        }
        None => {}
      }

      self.index_inc();

      let mut member_ast = ast::MemberAST::new(None, name);
      member_ast.push_node(member);
      structs_ast.push_member(&member_ast);
    }

    return Ok(Syntax::Struct(structs_ast));
  }
}
