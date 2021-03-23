use std::fs::File;
use std::io::prelude::*;

use super::super::super::interpreter::Interpreter;
use crate::error::result;

use crate::lexer::lexers;
use crate::parser::parsers;

impl Interpreter {
  pub(crate) fn import(&mut self, url:&str) -> Result<(), result::Error> {
    //TODO URLの参照位置を調整する
    let mut f: File;
    match File::open(url) {
      Ok(file) => {
        f = file;
      }

      Err(e) => {
        return Err(result::Error::FileReadError(e.to_string()));
      }
    }

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
      Ok(_) => {}
      Err(e) => {
        return Err(result::Error::FileReadError(e.to_string()));
      }
    }

    let mut lex = lexers::lex(&contents);
    let result = lex.run().get_tokens();
    println!("{:?}", result);

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run()?;
    println!("{:?}", result);

    self.push_scope();
    self.function_init(&result)?;

    return Ok(());
  }
}