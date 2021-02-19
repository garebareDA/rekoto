use super::super::ast::ast;
use super::super::parsers::Parsers;

impl Parsers {
  pub(crate) fn number(&mut self) -> Result<ast::Syntax, String> {
    let num: i64 = self
      .get_tokens(self.get_index())
      .get_value()
      .parse()
      .unwrap();
    let mut num_ast = ast::NumberAST::new(num);

    match self.formula_judge() {
      Some(formu) => match formu {
        Ok(objf) => {
          num_ast.push_node(&objf);
        }
        Err(e) => {
          return Err(e);
        }
      },
      None => {}
    }

    return Ok(ast::Syntax::Num(num_ast));
  }

  pub(crate) fn binary(&mut self) -> Result<ast::Syntax, String> {
    let ch = self
      .get_tokens(self.get_index())
      .get_value()
      .chars()
      .nth(0)
      .unwrap();
    let mut ch_ast = ast::BinaryAST::new(ch);

    match self.formula_judge() {
      Some(formu) => match formu {
        Ok(objf) => {
          ch_ast.push_node(&objf);
        }
        Err(e) => {
          return Err(e);
        }
      },
      None => {}
    }
    return Ok(ast::Syntax::Bin(ch_ast));
  }

  pub(crate) fn strings(&mut self) -> Result<ast::Syntax, String>{
    let strs = self
      .get_tokens(self.get_index())
      .get_value();
    let mut str_ast = ast::StringAST::new(strs);

    match self.formula_judge() {
      Some(formu) => match formu {
        Ok(obj) => {
          str_ast.push_node(&obj);
        }
        Err(e) => {
          return Err(e);
        }
      },
      None => {}
    }
    return Ok(ast::Syntax::Str(str_ast));
  }

  pub(crate) fn formula_judge(&mut self) -> Option<Result<ast::Syntax, String>> {
    if self.get_index() as usize >= self.get_tokens_len() - 1 {
      return None;
    }

    self.index_inc();
    let judge = self.judge();
    match judge {
      Some(obj) => match obj {
        Ok(syn) => match syn {
          ast::Syntax::Bin(bin) => {
            return Some(Ok(ast::Syntax::Bin(bin)));
          }

          ast::Syntax::Num(num) => {
            return Some(Ok(ast::Syntax::Num(num)));
          }

          ast::Syntax::Str(strs) => {
            return Some(Ok(ast::Syntax::Str(strs)));
          }

          ast::Syntax::Var(var) => {
            if var.get_is_def() {
              return Some(Err(format!("syntax error let")));
            }
            return Some(Ok(ast::Syntax::Var(var)));
          }

          ast::Syntax::Call(call) => {
            return Some(Ok(ast::Syntax::Call(call)));
          }
        },
        Err(e) => {
          return Some(Err(e));
        }
      },
      None => {
        return None;
      }
    }
  }
}
