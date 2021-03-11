use std::env;
use std::fs::File;
use std::io::prelude::*;

use crate::error::result;
use crate::interpreters::interpreter;
use crate::lexer::lexers;
use crate::parser::parsers;

pub fn read_file() -> Result<(), result::Error> {
  let args: Vec<String> = env::args().collect();
  if args.len() > 3 && 3 > args.len() {
    return Err(result::Error::FileReadError("too many args".to_string()));
  }

  let query = &args[1];
  if query == "run" {
    let filename = &args[2];
    if filename == "" {
      return Err(result::Error::FileReadError("file is empty".to_string()));
    }

    let mut f: File;
    match File::open(filename) {
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

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        let mut interpreter = interpreter::Interpreter::new();
        return interpreter.run(result);
      }

      Err(e) => {
        return Err(e);
      }
    }
  }

  return Err(result::Error::FileReadError(format!(
    "{} command is not found",
    query
  )));
}
