pub mod interpreter;
pub mod judge;

#[cfg(test)]
mod tests {
  use crate::interpreters::interpreter;
  use crate::lexer::lexers;
  use crate::parser::parsers;

  #[test]
  fn call_print() {
    let mut lex = lexers::lex("print(\"hello world!\");");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    match result {
      Ok(result) => {
        let mut interpreter = interpreter::Interpreter::new();
        match interpreter.debug_run(result) {
          Ok(result) => {
            if result[0] != "hello world!" {
              panic!();
            }
          }
          Err(e) => {
            panic!(e);
          }
        }
      }

      Err(e) => {
        panic!(e);
      }
    }
  }

  #[test]
  fn variable() {
    let mut lex = lexers::lex("let a = \"hello world!\";\nprint(a);");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    match result {
      Ok(result) => {
        let mut interpreter = interpreter::Interpreter::new();
        match interpreter.debug_run(result) {
          Ok(result) => {
            if result[0] != "hello world!" {
              panic!();
            }
          }
          Err(e) => {
            panic!(e);
          }
        }
      }

      Err(e) => {
        panic!(e);
      }
    }
  }
}
