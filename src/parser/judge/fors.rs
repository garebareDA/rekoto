use super::super::ast::{ast, ast::Node, ast::Syntax};
use super::super::parsers::Parsers;

impl Parsers {
  pub(crate) fn fors(&mut self) -> Result<Syntax, String> {
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
              return Err("var initlize error".to_string());
            }
          }

          _ => {
            return Err("for initlize error".to_string());
          }
        },
        Err(e) => {
          return Err(e);
        }
      },
      None => {
        return Err("for initlize error".to_string());
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
            return Err("for jdugement error".to_string());
          }
        },

        Err(e) => {
          return Err(e);
        }
      },

      None => {
        return Err("for judgement error".to_string());
      }
    }

    self.index_inc();
    match self.judge() {
      Some(judge) => {
        match judge {
          Ok(obj) => {
            match obj {
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
                return Err("for add error".to_string());
              }
            }
          }

          Err(e) => {
            return Err(e);
          }
        }
      }

      None => {
        return Err("for add error".to_string());
      }
    }

    let mut fors = ast::ForsAST::new(init, judges, add);
    self.index_inc();
    match self.judge() {
      Some(judge) => {
        match judge {
          Ok(obj) => {
            match obj {
              ast::Syntax::Bin(bin) => {
                return Err(format!("{} syntax error", bin.get_bin()))
              }
              _ => {}
            }
            fors.push_node(obj);
          }

          Err(e) => {
            return Err(e);
          }
        }
      }
      None => {}
    }

    return Ok(Syntax::For(Box::new(fors)));
  }
}
