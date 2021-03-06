use lelex;

use super::token;

static TOKEN: token::Token = token::Token::new();

pub fn lex(word: &str) -> lelex::lexers::Lexer {
  let mut lex = lelex::lexers::Lexer::new(word);
  lex.push_one_word(TOKEN._comment, "#").unwrap();
  lex.push_reserved_word(TOKEN._let, "let").unwrap();
  lex.push_reserved_word(TOKEN._const, "const").unwrap();
  lex.push_reserved_word(TOKEN._if, "if").unwrap();
  lex.push_reserved_word(TOKEN._else, "else").unwrap();
  lex.push_reserved_word(TOKEN._elif, "elif").unwrap();
  lex.push_reserved_word(TOKEN._for, "for").unwrap();
  lex.push_reserved_word(TOKEN._fn, "fn").unwrap();
  lex.push_reserved_word(TOKEN._import, "import").unwrap();
  lex.push_reserved_word(TOKEN._return, "return").unwrap();
  lex.push_reserved_word(TOKEN._break, "break").unwrap();
  lex.push_reserved_word(TOKEN._true, "true").unwrap();
  lex.push_reserved_word(TOKEN._false, "false").unwrap();
  lex.push_reserved_word(TOKEN._struct, "struct").unwrap();
  lex.push_between_ward(TOKEN._string, "\"").unwrap();
  lex.push_between_ward(TOKEN._string, "\'").unwrap();
  lex.set_number_token(TOKEN._number).unwrap();
  lex.set_other_token(TOKEN._variable).unwrap();
  return lex;
}