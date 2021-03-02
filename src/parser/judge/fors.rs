use super::super::ast::{ast, ast::Node, ast::Syntax};
use super::super::parsers::Parsers;
use crate::error::result;

impl Parsers {
  pub(crate) fn fors(&mut self) -> Result<Syntax, result::Error> {
    let init: Syntax;
    let judges: Syntax;
    let add: Syntax;

    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          Syntax::Var(var) => {
            if var.get_node_len() > 0 {
              init = Syntax::Var(var);
            } else {
              return Err(result::Error::SyntaxError(format!(
                "var {} initlize error",
                var.get_name()
              )));
            }
          }

          _ => {
            return Err(result::Error::SyntaxError("for initlize error".to_string()));
          }
        },
        Err(e) => {
          return Err(e);
        }
      },
      None => {
        return Err(result::Error::SyntaxError("for initlize error".to_string()));
      }
    }

    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          Syntax::Num(num) => {
            judges = Syntax::Num(num);
          }

          Syntax::Str(strs) => {
            judges = Syntax::Str(strs);
          }

          Syntax::Call(call) => {
            judges = Syntax::Call(call);
          }

          Syntax::Var(var) => {
            judges = Syntax::Var(var);
          }

          _ => {
            return Err(result::Error::SyntaxError(
              "for initlize errorjdugement error".to_string(),
            ));
          }
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err(result::Error::SyntaxError(
          "for judgement error".to_string(),
        ));
      }
    }

    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          Syntax::Num(num) => {
            add = Syntax::Num(num);
          }

          Syntax::Str(strs) => {
            add = Syntax::Str(strs);
          }

          Syntax::Call(call) => {
            add = Syntax::Call(call);
          }

          Syntax::Var(var) => {
            add = Syntax::Var(var);
          }

          _ => {
            return Err(result::Error::SyntaxError("for formula error".to_string()));
          }
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err(result::Error::SyntaxError("for formula error".to_string()));
      }
    }

    let mut fors = ast::ForsAST::new(init, judges, add);
    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => {
          match obj {
            ast::Syntax::Bin(bin) => {
              return Err(result::Error::SyntaxError(format!(
                "{} syntax error",
                bin.get_bin()
              )))
            }
            _ => {}
          }
          fors.push_node(obj);
        }

        Err(e) => {
          return Err(e);
        }
      },
      None => {}
    }

    return Ok(Syntax::For(Box::new(fors)));
  }
}
