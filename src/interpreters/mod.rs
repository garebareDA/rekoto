pub mod interpreter;
pub mod judge;
pub mod variables;
pub mod functions;

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
    assert_eq!(
      result("let a = 'hello' + 'world!' + 2;\nprint(a);"),
      "helloworld!2"
    );
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
    assert_eq!(
      result("if 0 == 1 {print('ok')} else {print('else')}"),
      "else"
    );
    assert_eq!(
      result("if 0 == 1 {print('ok')} elif 0 == 0 {print('elif')}"),
      "elif"
    );
    assert_eq!(
      result("if 0 == 1 {print('ok')} elif 1 == 0 {print('elif')} else {print('else')}"),
      "else"
    );
    assert_eq!(
      result("if 0 == 1 {print('ok')} elif 1 == 0 {print('elif')} else { } print('ok')"),
      "ok"
    );
  }

  #[test]
  fn var() {
    assert_eq!(result("let a = 1 + 1; a = 1; print(a);"), "1");
  }

  #[test]
  fn fors() {
    assert_eq!(
      result("for let i = 0; i < 5; i++; {print('for')}"),
      "for"
    );
  }

  #[test]
  fn function_call() {
    assert_eq!(
      result_not_main(
        "fn main() {
      let a = test(1 + 1, 1) + 1;
      print(a);
    }
    fn test(a:number, b:number):number {
      let c = a + b;
      print(c);
      return c + 1;
    }"
      ),
      "5"
    );
  }

  #[test]
  fn function_call_if() {
    assert_eq!(
      result_not_main(
        "fn main() {
          let a = test(1 + 1, 1) + 1;
          print(a);
        }
        fn test(a:number, b:number):number {
          let c = a + b;
          print(c);
          if 1 == 1 {
            return c + 2;
          }
          return c + 1;
        }"
      ),
      "6"
    );
  }

  fn result(syn: &str) -> String {
    let run = format!("fn main() {{ {} }}", syn);
    let mut lex = lexers::lex(&run);
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

  fn result_not_main(syn: &str) -> String {
    let mut lex = lexers::lex(&syn);
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
