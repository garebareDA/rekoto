use lelex;

use super::token;

static TOKEN: token::Token = token::Token::new();

pub fn lex(word: &str) -> lelex::lexers::Lexer {
  let mut lex = lelex::lexers::Lexer::new(word);
  lex.push_reserved_word(TOKEN._let, "let").unwrap();
  lex.push_reserved_word(TOKEN._const, "const").unwrap();
  lex.push_reserved_word(TOKEN._if, "if").unwrap();
  lex.push_reserved_word(TOKEN._else, "else").unwrap();
  lex.push_reserved_word(TOKEN._elif, "elif").unwrap();
  lex.push_reserved_word(TOKEN._for, "for").unwrap();
  lex.push_reserved_word(TOKEN._fn, "fn").unwrap();
  lex.push_reserved_word(TOKEN._import, "import").unwrap();
  lex.push_reserved_word(TOKEN._greater_equ, "=>").unwrap();
  lex.push_reserved_word(TOKEN._less_equ, "=<").unwrap();
  lex.push_reserved_word(TOKEN._equ, "==").unwrap();
  lex.push_reserved_word(TOKEN._not_equ, "!=").unwrap();
  lex.push_reserved_word(TOKEN._or, "||").unwrap();
  lex.push_reserved_word(TOKEN._and, "&&").unwrap();
  lex.push_between_ward(TOKEN._string, "\"").unwrap();
  lex.push_reserved_word(TOKEN._string, "\'").unwrap();
  lex.set_number_token(TOKEN._number).unwrap();
  lex.set_other_token(TOKEN._variable).unwrap();
  return lex;
}