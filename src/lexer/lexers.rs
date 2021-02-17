use lelex;

use super::token;

static TOKEN: token::Token = token::Token::new();

pub fn lex(word: &str) -> lelex::lexers::Lexer {
  let mut lex = lelex::lexers::Lexer::new(word);
  lex.push_reserved_word(TOKEN._let, "let").unwrap();
  lex.push_reserved_word(TOKEN._const, "const").unwrap();
  lex.set_number_token(TOKEN._number).unwrap();
  lex.set_other_token(TOKEN._variable).unwrap();
  return lex;
}