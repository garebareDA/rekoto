pub mod interpreter;
pub mod judge;

#[cfg(test)]
mod tests {
  use crate::interpreters::interpreter;
  use crate::lexer::lexers;
  use crate::parser::parsers;

  #[test]
  fn call_print() {
    assert_eq!(result("print(\"hello world!\");"), "hello world!");
  }

  #[test]
  fn variable() {
    assert_eq!(
      result("let a = \"hello world!\";\nprint(a);"),
      "hello world!"
    );
  }

  #[test]
  fn four_arithmetic() {
    assert_eq!(result("let a = 1 + 1 - 1 / 1 % 1 * 5;\nprint(a);"), "2");
  }

  #[test]
  fn string_concat() {
    assert_eq!(result("let a = 'hello' + 'world!' + 2;\nprint(a);"), "helloworld!2");
  }

  #[test]
  fn comparison_operator() {
    assert_eq!(result("let a = 1 < 0;\nprint(a);"), "false");
    assert_eq!(result("let a = 0 < 1;\nprint(a);"), "true");
    assert_eq!(result("let a = 1 <= 0;\nprint(a);"), "false");
    assert_eq!(result("let a = 0 <= 0;\nprint(a);"), "true");
    assert_eq!(result("let a = 1 > 0;\nprint(a);"), "true");
    assert_eq!(result("let a = 0 > 1;\nprint(a);"), "false");
    assert_eq!(result("let a = 0 >= 0;\nprint(a);"), "true");
    assert_eq!(result("let a = 0 >= 1;\nprint(a);"), "false");
  }

  #[test]
  fn eq_noeq() {
    assert_eq!(result("let a = 1 == 1;\nprint(a);"), "true");
    assert_eq!(result("let a = 0 != 1;\nprint(a);"), "true");
  }

  #[test]
  fn or_and() {
    assert_eq!(result("let a = true || false;\nprint(a);"), "true");
    assert_eq!(result("let a = true && true;\nprint(a);"), "true");
  }

  #[test]
  fn ifs() {
    assert_eq!(result("if 0 < 1 {print('ok')}"), "ok");
    assert_eq!(result("if 0 == 1 {print('ok')} else {print('else')}"), "else");
    assert_eq!(result("if 0 == 1 {print('ok')} elif 0 == 0 {print('elif')}"), "elif");
    assert_eq!(result("if 0 == 1 {print('ok')} elif 1 == 0 {print('elif')} else {print('else')}"), "else");
    assert_eq!(result("if 0 == 1 {print('ok')} elif 1 == 0 {print('elif')} else { } print('ok')"), "ok");
  }

  fn result(syn: &str) -> String {
    let mut lex = lexers::lex(syn);
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    match result {
      Ok(result) => {
        let mut interpreter = interpreter::Interpreter::new();
        match interpreter.debug_run(result) {
          Ok(result) => {
            return result[0].to_string();
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
