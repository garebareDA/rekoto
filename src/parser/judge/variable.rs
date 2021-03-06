use super::super::super::lexer::token;
use super::super::ast::{ast, ast::Node, ast::Syntax, ast::Type};
use super::super::parsers::{ParseState, Parsers};
use crate::error::result;
static TOKEN: token::Token = token::Token::new();

impl Parsers {
  pub(crate) fn variable_def(
    &mut self,
    is_mutable: bool,
    is_def: bool,
  ) -> Result<Syntax, result::Error> {
    //letまたはconstの変数名を取得
    self.push_state(ParseState::Var);

    let judge = self
      .judge()
      .ok_or(result::Error::SyntaxError(format!("syntax error variable")))?;
    match judge? {
      ast::Syntax::Var(mut var) => {
        var.set_is_def(is_def);
        var.set_is_mutable(is_mutable);

        /*
        lec const のキワードが : かどうか
        : だった場合に型を設定
        */
        var.set_type(self.check_types()?);

        //let const キーワードの次が = かどうか
        match self.variable_def_inspect() {
          Ok(()) => {
            self.index_inc();
          }
          Err(_) => return Ok(ast::Syntax::Var(var)),
        }

        self.pop_state();

        let judge = self
          .judge()
          .ok_or(result::Error::SyntaxError(format!("syntax error variable")))?;
        match judge {
          Ok(syn) => {
            //変数の中身を入れる
            match syn {
              ast::Syntax::Var(var_2) => {
                let ast = ast::Syntax::Var(var_2);
                var.push_node(ast);
                return Ok(ast::Syntax::Var(var));
              }

              ast::Syntax::Bin(bin) => {
                let ast = ast::Syntax::Bin(bin);
                var.push_node(ast);
                return Ok(ast::Syntax::Var(var));
              }

              ast::Syntax::Num(num) => {
                let ast = ast::Syntax::Num(num);
                var.push_node(ast);
                return Ok(ast::Syntax::Var(var));
              }

              ast::Syntax::Str(strs) => {
                let ast = ast::Syntax::Str(strs);
                var.push_node(ast);
                return Ok(ast::Syntax::Var(var));
              }

              ast::Syntax::Call(call) => {
                let ast = ast::Syntax::Call(call);
                var.push_node(ast);
                return Ok(ast::Syntax::Var(var));
              }

              ast::Syntax::Bool(bools) => {
                let ast = ast::Syntax::Bool(bools);
                var.push_node(ast);
                return Ok(ast::Syntax::Var(var));
              }

              ast::Syntax::Scope(_) => {
                return Err(result::Error::SyntaxError(format!(
                  "invalid scope variable {}",
                  var.get_name()
                )));
              }

              ast::Syntax::Struct(structs) => {
                let ast = ast::Syntax::Struct(structs);
                var.push_node(ast);
                return Ok(ast::Syntax::Var(var));
              }

              _ => {
                return Err(result::Error::SyntaxError(format!(
                  "syntax error scope {}",
                  var.get_name()
                )));
              }
            }
          }

          Err(e) => {
            return Err(result::Error::SyntaxError(format!(
              "syntax error variable {} \n {}",
              var.get_name(),
              e
            )));
          }
        }
      }

      ast::Syntax::Num(num) => {
        return Err(result::Error::SyntaxError(format!(
          "{} cannot be used for variables",
          num.get_num()
        )));
      }

      ast::Syntax::Bin(bin) => {
        return Err(result::Error::SyntaxError(format!(
          "{} cannot be used for variables",
          bin.get_bin()
        )))
      }

      ast::Syntax::Str(strs) => {
        return Err(result::Error::SyntaxError(format!(
          "{} cannot be used for variables",
          strs.get_str()
        )))
      }

      ast::Syntax::Call(call) => {
        return Err(result::Error::SyntaxError(format!(
          "{} cannot be used for variables",
          call.get_name()
        )))
      }

      ast::Syntax::Scope(_) => {
        return Err(result::Error::SyntaxError(format!(
          "`{{` cannot be used for variables"
        )))
      }

      _ => return Err(result::Error::SyntaxError(format!("syntax error scope"))),
    }
  }

  fn variable_def_inspect(&mut self) -> Result<(), result::Error> {
    self.index_inc();
    let judge = self
      .judge()
      .ok_or(result::Error::SyntaxError(format!("syntax error variable")))?;
    match judge? {
      ast::Syntax::Bin(bin) => {
        let bin = bin.get_bin();
        if bin == "=" {
          return Ok(());
        }
        return Err(result::Error::SyntaxError(format!(
          "Only the = operator can be used for assignment"
        )));
      }
      _ => {
        return Err(result::Error::SyntaxError(format!(
          "Only the = operator can be used for assignment"
        )));
      }
    }
  }

  pub(crate) fn variable(&mut self, is_def: bool) -> Result<ast::Syntax, result::Error> {
    let name: &str;
    match self.get_tokens(self.get_index()) {
      Some(tokens) => {
        name = tokens.get_value();
        if name == "" {
          return Err(result::Error::SyntaxError(format!(
            "{} variable name error",
            name
          )));
        }
      }

      None => {
        return Err(result::Error::SyntaxError(
          "syntax error variable possible parser bug".to_string(),
        ));
      }
    };

    let mut ast = ast::VariableAST::new(name, false, is_def);

    //関数の呼び出しの判定 ( がるか
    match self.get_tokens(self.get_index() + 1) {
      Some(tokens) => {
        let verification_token = tokens.get_token();

        if verification_token == TOKEN._paren_left && self.get_last_state() != &ParseState::Function
        {
          self.push_state(ParseState::Call);
          let judge = self.call();
          self.pop_state();
          return judge;
        }

        if verification_token == TOKEN._equal && self.get_last_state() != &ParseState::Var {
          return self.variable_def(true, false);
        }

        if verification_token == TOKEN._braces_left && self.get_last_state() == &ParseState::Struct
        {
          return Ok(ast::Syntax::Var(ast));
        }

        if verification_token == TOKEN._braces_left {
          self.push_state(ParseState::New);
          //インスタンスを返す
          return self.instance();
        }
      }

      None => {}
    };

    if self.get_last_state() == &ParseState::Var
      || self.get_last_state() == &ParseState::Member
      || self.get_last_state() == &ParseState::New
    {
      return Ok(ast::Syntax::Var(ast));
    }

    match self.formula_judge() {
      Some(formu) => {
        ast.push_node(formu?);
      }
      None => {}
    }
    return Ok(ast::Syntax::Var(ast));
  }

  pub(crate) fn check_types(&mut self) -> Result<Option<ast::Types>, result::Error> {
    let tokens = self
      .get_tokens(self.get_index() + 1)
      .ok_or(result::Error::SyntaxError("Syntax error type".to_string()))?;

    if tokens.get_token() != TOKEN._colon {
      if self.get_last_state() == &ParseState::Function {
        return Err(result::Error::SyntaxError(
          "param not found type".to_string(),
        ));
      }
      return Ok(None);
    }

    self.index_add(2);
    let tokens = self
      .get_tokens(self.get_index())
      .ok_or(result::Error::SyntaxError("Syntax error type".to_string()))?;
    let types = tokens.get_value();
    if types == "number" {
      return Ok(Some(ast::Types::Number));
    } else if types == "string" {
      return Ok(Some(ast::Types::String));
    } else if types == "bool" {
      return Ok(Some(ast::Types::Bool));
    }

    return Ok(Some(ast::Types::Struct));
  }
}
