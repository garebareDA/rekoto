use super::super::ast::ast;
use super::super::ast::ast::Syntax;
use super::super::parsers::Parsers;

impl Parsers {
  pub(crate) fn ifs(&mut self) -> Result<ast::Syntax, String> {
    let mut ifs_ast;
    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          Syntax::Str(strs) => {
            ifs_ast = ast::IfsAST::new(Syntax::Str(strs));
          }

          Syntax::Num(num) => {
            ifs_ast = ast::IfsAST::new(Syntax::Num(num));
          }

          _ => {
            return Err("if syntax error".to_string());
          }
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err(format!("if syntax error"));
      }
    }

    self.index_inc();
    match self.judge() {
      Some(judge) => {
        match judge {
          Ok(obj) => {
            match obj {
              Syntax::Bin(_) => {
                return Err("if scope error".to_string());
              }

              _ => {
                ifs_ast.push_scope(&obj);
                return Ok(Syntax::Ifs(Box::new(ifs_ast)));
              }
            }
          }

          Err(e) => {
            return Err(e);
          }
        }
      }

      None => {
        return Err("if scope error".to_string());
      }
    }
  }
}
