use super::super::ast::{ast, ast::Node, ast::Syntax};
use super::super::parsers::Parsers;
use crate::error::result;

impl Parsers {
  pub(crate) fn ifs(&mut self) -> Result<ast::Syntax, result::Error> {
    let syntax = self.if_judge()?;
    let scope = self.if_scope()?;
    let mut ifs_ast = ast::IfsAST::new(syntax);
    ifs_ast.push_node(scope);
    return Ok(Syntax::Ifs(Box::new(ifs_ast)));
  }

  pub(crate) fn elses(&mut self) -> Result<ast::Syntax, result::Error> {
    let syntax = self.if_scope()?;
    let mut else_ast = ast::ElseAST::new();
    else_ast.push_node(syntax);
    return Ok(Syntax::Else(Box::new(else_ast)));
  }

  pub(crate) fn elif(&mut self) -> Result<ast::Syntax, result::Error> {
    let syntax = self.if_judge()?;
    let scope = self.if_scope()?;
    let mut elif_ast = ast::ElifAST::new(syntax);
    elif_ast.push_node(scope);
    return Ok(Syntax::Elif(Box::new(elif_ast)));
  }

  fn if_judge(&mut self) -> Result<ast::Syntax, result::Error> {
    self.index_inc();
    let judge = self.judge().ok_or(result::Error::SyntaxError(format!(
      "if syntax error possible parser bug"
    )))?;
    match judge? {
      Syntax::Str(strs) => {
        return Ok(Syntax::Str(strs));
      }

      Syntax::Num(num) => {
        return Ok(Syntax::Num(num));
      }

      Syntax::Var(var) => {
        return Ok(Syntax::Var(var));
      }

      _ => {
        return Err(result::Error::SyntaxError(
          "if syntax error must be string number boolean variable".to_string(),
        ));
      }
    }
  }

  fn if_scope(&mut self) -> Result<ast::Syntax, result::Error> {
    self.index_inc();
    let judge = self.judge().ok_or(result::Error::SyntaxError(
      "if scope error possible parser bug".to_string(),
    ))??;
    match judge {
      Syntax::Bin(bin) => {
        return Err(result::Error::SyntaxError(format!(
          "if scope {} syntax error",
          bin.get_bin()
        )));
      }

      _ => {
        return Ok(judge);
      }
    }
  }
}
