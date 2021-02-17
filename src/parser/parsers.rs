extern crate lelex;
use super::ast;

pub struct Parsers {
  pub tokens: Vec<lelex::tokens::Tokens>,
  index: i64,
}

impl Parsers {
  pub fn new(tokens: Vec<lelex::tokens::Tokens>) -> Self {
    Self { tokens, index: 0 }
  }

  pub fn run(&mut self) -> Result<ast::ast::RootAST, String> {
    let mut root = ast::ast::RootAST::new();
    let len = self.tokens.len();

    loop {
      match self.judge() {
        Some(result) => match result {
          Ok(ast) => {
            root.push_node(ast);
          }
          Err(e) => {
            return Err(e);
          }
        },
        None => {}
      }

      self.index_inc();
      if len <= self.index as usize {
        break;
      }
    }

    return Ok(root);
  }

  pub(crate) fn get_index(&self) -> i64 {
    self.index
  }

  fn index_inc(&mut self) {
    self.index += 1;
  }

  fn index_add(&mut self, index: usize) {
    self.index += index as i64;
  }

  pub(crate) fn get_tokens(&self, num: i64) -> &lelex::tokens::Tokens {
    return &self.tokens[num as usize];
  }
}
