use super::super::ast::{ast, ast::Node, ast::Syntax};
use super::super::parsers::ParseState;
use super::super::parsers::Parsers;
use crate::error::result;


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
    println!("{:?}", self.get_tokens(self.get_index()));
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
          if self.get_last_state() != &ParseState::Struct {
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
}
