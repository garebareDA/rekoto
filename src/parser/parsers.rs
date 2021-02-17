extern crate lelex;

use super::super::lexer::token;
use super::ast;

static TOKEN: token::Token = token::Token::new();

pub struct Persers {
  tokens: Vec<lelex::tokens::Tokens>,
  index: i64,
}

impl Persers {
  pub fn new(tokens: Vec<lelex::tokens::Tokens>) -> Self {
    Self { tokens, index: 0 }
  }

  pub fn run(&mut self) -> ast::ast::RootAST {
    let mut root = ast::ast::RootAST::new();
    let len = self.tokens.len();
    loop {
      let token = self.get_tokens(self.index).get_token();

      self.index_inc();
      if len <= self.index as usize {
        break;
      }
    }

    return root;
  }

  fn index_inc(&mut self) {
    self.index += 1;
  }

  fn get_tokens(&self, num: i64) -> &lelex::tokens::Tokens {
    return &self.tokens[num as usize];
  }
}
