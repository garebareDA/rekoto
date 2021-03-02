use super::super::ast::{ast, ast::Node, ast::Syntax};
use super::super::parsers::Parsers;
use crate::error::result;

impl Parsers {
  pub(crate) fn ifs(&mut self) -> Result<ast::Syntax, result::Error> {
    match self.if_judge() {
      Ok(syntax) => {
        let mut ifs_ast = ast::IfsAST::new(syntax);
        match self.if_scope() {
          Ok(syntax) => {
            ifs_ast.push_node(syntax);
            return Ok(Syntax::Ifs(Box::new(ifs_ast)));
          }

          Err(e) => {
            return Err(e);
          }
        }
      }

      Err(e) => {
        return Err(e);
      }
    }
  }

  pub(crate) fn elses(&mut self) -> Result<ast::Syntax, result::Error> {
    match self.if_scope() {
      Ok(syntax) => {
        let mut else_ast = ast::ElseAST::new();
        else_ast.push_node(syntax);
        return Ok(Syntax::Else(Box::new(else_ast)));
      }

      Err(e) => {
        return Err(e);
      }
    }
  }

  pub(crate) fn elif(&mut self) -> Result<ast::Syntax, result::Error> {
    match self.if_judge() {
      Ok(syntax) => {
        let mut elif_ast = ast::ElifAST::new(syntax);
        match self.if_scope() {
          Ok(syntax) => {
            elif_ast.push_node(syntax);
            return Ok(Syntax::Elif(Box::new(elif_ast)));
          }

          Err(e) => {
            return Err(e);
          }
        }
      }

      Err(e) => {
        return Err(e);
      }
    }
  }

  fn if_judge(&mut self) -> Result<ast::Syntax, result::Error> {
    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          Syntax::Str(strs) => {
            return Ok(Syntax::Str(strs));
          }

          Syntax::Num(num) => {
            return Ok(Syntax::Num(num));
          }

          _ => {
            return Err(result::Error::SyntaxError(
              "if syntax error must be string or number boolean".to_string(),
            ));
          }
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err(result::Error::SyntaxError(format!(
          "if syntax error possible parser bug"
        )));
      }
    }
  }

  fn if_scope(&mut self) -> Result<ast::Syntax, result::Error> {
    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          Syntax::Bin(bin) => {
            return Err(result::Error::SyntaxError(format!(
              "if scope {} syntax error",
              bin.get_bin()
            )));
          }

          _ => {
            return Ok(obj);
          }
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err(result::Error::SyntaxError(
          "if scope error possible parser bug".to_string(),
        ));
      }
    }
  }
}
