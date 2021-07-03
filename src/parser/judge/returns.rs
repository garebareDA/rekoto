use super::super::ast::{ast, ast::Node};
use super::super::parsers::Parsers;
use crate::error::result;

impl Parsers {
  pub(crate) fn returns(&mut self) -> Result<ast::Syntax, result::Error> {
    self.index_inc();
    match self.judge() {
      Some(judge) => match judge? {
        ast::Syntax::Var(var) => {
          let mut return_ast = ast::ReturnAST::new();
          return_ast.push_node(ast::Syntax::Var(var));
          return Ok(ast::Syntax::Return(Box::new(return_ast)));
        }

        ast::Syntax::Num(num) => {
          let mut return_ast = ast::ReturnAST::new();
          return_ast.push_node(ast::Syntax::Num(num));
          return Ok(ast::Syntax::Return(Box::new(return_ast)));
        }

        ast::Syntax::Str(strs) => {
          let mut return_ast = ast::ReturnAST::new();
          return_ast.push_node(ast::Syntax::Str(strs));
          return Ok(ast::Syntax::Return(Box::new(return_ast)));
        }

        ast::Syntax::Call(call) => {
          let mut return_ast = ast::ReturnAST::new();
          return_ast.push_node(ast::Syntax::Call(call));
          return Ok(ast::Syntax::Return(Box::new(return_ast)));
        }

        _ => {
          return Err(result::Error::SyntaxError(
            "return syntax error".to_string(),
          ))
        }
      },

      None => {
        let return_ast = ast::ReturnAST::new();
        return Ok(ast::Syntax::Return(Box::new(return_ast)));
      }
    }
  }
}
