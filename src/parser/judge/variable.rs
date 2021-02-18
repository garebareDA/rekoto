use super::super::ast::ast;
use super::super::parsers::{Parsers, ParseState};

impl Parsers {
  pub(crate) fn variable_def(&mut self, is_mutable: bool) -> Result<ast::Syntax, String> {
    //letまたはconstの変数を取得
    match self.variable_def_inner() {
      Ok(syn) => match syn {
        ast::Syntax::Var(mut var) => {
          var.set_is_def(true);
          var.set_is_mutable(is_mutable);
          self.index_inc();

          //let const キーワードの次が = かどうか
          match self.variable_def_inspect() {
            Ok(()) => {}
            Err(e) => return Err(e),
          }

          self.index_inc();
          match self.variable_def_inner() {
            Ok(syn) => {
              //変数の中身を入れる
              match syn {
                ast::Syntax::Var(var_2) => {
                  let ast = ast::Syntax::Var(var_2);
                  var.push_node(&ast);
                  return Ok(ast::Syntax::Var(var));
                }

                ast::Syntax::Bin(bin) => {
                  let ast = ast::Syntax::Bin(bin);
                  var.push_node(&ast);
                  return Ok(ast::Syntax::Var(var));
                }

                ast::Syntax::Num(num) => {
                  let ast = ast::Syntax::Num(num);
                  var.push_node(&ast);
                  return Ok(ast::Syntax::Var(var));
                }

                ast::Syntax::Str(strs) => {
                  let ast = ast::Syntax::Str(strs);
                  var.push_node(&ast);
                  return Ok(ast::Syntax::Var(var));
                }
              }
            }

            Err(e) => {
              return Err(e);
            }
          }
        }

        ast::Syntax::Num(num) => {
          return Err(format!("{} can be used for variables", num.get_num()));
        }

        ast::Syntax::Bin(bin) => {
          return Err(format!("{} can be used for variables", bin.get_bin()))
        }

        ast::Syntax::Str(strs) => {
          return Err(format!("{} can be used for variables",strs.get_str()))
        }
      },

      Err(s) => {
        return Err(s);
      }
    }
  }

  //変数を取得
  fn variable_def_inner(&mut self) -> Result<ast::Syntax, String> {
    match self.judge() {
      Some(syn) => match syn {
        Ok(obj) => {
          return Ok(obj);
        }

        Err(s) => {
          return Err(s);
        }
      },

      None => return Err(format!("syntax error")),
    }
  }

  fn variable_def_inspect(&mut self) -> Result<(), String> {
    match self.variable_def_inner() {
      Ok(syn) => match syn {
        ast::Syntax::Bin(bin) => {
          let bin = bin.get_bin();
          if bin == '=' {
            return Ok(());
          }
          return Err(format!("Only the = operator can be used for assignment"));
        }
        _ => {
          return Err(format!("Only the = operator can be used for assignment"));
        }
      },

      Err(e) => {
        return Err(e);
      }
    }
  }

  pub(crate) fn variable(&mut self, is_def: bool) -> Result<ast::Syntax, String> {
    let name = self.get_tokens(self.get_index()).get_value();
    if name != "" {
      let mut ast = ast::VariableAST::new(name, false, is_def);
      if self.get_last_state() == &ParseState::Var {
        return Ok(ast::Syntax::Var(ast));
      }

      match self.formula_judge() {
        Some(formu) => match formu {
          Ok(obj) => {
            ast.push_node(&obj);
          }
          Err(e) => {
            return Err(e);
          }
        },
        None => {}
      }

      return Ok(ast::Syntax::Var(ast));
    }

    return Err(format!("{} variable name error", name));
  }
}
