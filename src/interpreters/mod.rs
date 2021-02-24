pub mod interpreter;
pub mod judge;

#[cfg(test)]
mod tests {
  use crate::lexer::lexers;
  use crate::parser::parsers;
  use crate::interpreters::interpreter;

  #[test]
  fn call_print() {
    let mut lex = lexers::lex("print(\"hello world!\");");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    match result {
        Ok(result) => {
            let mut interpreter = interpreter::Interpreter::new();
            match interpreter.run(result) {
                Ok(_) => {
                  if interpreter.get_out() != "hello world!" {
                    panic!();
                  }
                }
                Err(_) => {
                    panic!();
                }
            }
        }

        Err(_) => {
          panic!();
        }
    }
  }
}