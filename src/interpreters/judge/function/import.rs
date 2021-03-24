use std::fs::File;
use std::io::prelude::*;

use super::super::super::interpreter::Interpreter;
use crate::error::result;

use crate::lexer::lexers;
use crate::parser::parsers;

use std::path::{Path, PathBuf};

impl Interpreter {
  pub(crate) fn import(&mut self, path: &str) -> Result<(), result::Error> {
    let my_path = Path::new(self.get_path());
    let parent = my_path.parent();
    let join_path:PathBuf;

    match parent {
      Some(p) => {
        join_path = p.join(path);
        println!("join_path {}", join_path.display());
      }

      None => {
        return Err(result::Error::InterpreterError(format!(
          "{} is not found",
          my_path.display()
        )))
      }
    }

    let mut f: File;
    match File::open(join_path) {
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
