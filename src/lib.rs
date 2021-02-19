pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
  use super::lexer::lexers;
  use super::parser::{ast, parsers};
  #[test]
  fn formula() {
    let mut lex = lexers::lex("let a = 1 + 1;");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    match result {
      Ok(result) => {
        for obj in result.get_node().iter() {
          match obj {
            ast::ast::Syntax::Var(var) => {
              if var.get_name() != "a" {
                panic!();
              }

              match var.get_node_index(0) {
                ast::ast::Syntax::Num(num) => {
                  if num.get_num() != 1 {
                    panic!();
                  }

                  match num.get_node_index(0) {
                    ast::ast::Syntax::Bin(bin) => {
                      if bin.get_bin() != '+' {
                        panic!();
                      }

                      match bin.get_node_index(0) {
                        ast::ast::Syntax::Num(num) => {
                          if num.get_num() != 1 {
                            panic!();
                          }
                        }

                        _ => {
                          panic!();
                        }
                      }
                    }

                    _ => {
                      panic!();
                    }
                  }
                }

                _ => {
                  panic!();
                }
              }
            }
            _ => {
              panic!()
            }
          }
        }
      }

      Err(e) => {
        panic!(e);
      }
    }
  }

  #[test]
  fn strings() {
    let mut lex = lexers::lex("let a = \"string\";");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node().iter() {
          match obj {
            ast::ast::Syntax::Var(var) => {
              if var.get_name() != "a" {
                panic!();
              }

              match var.get_node_index(0) {
                ast::ast::Syntax::Str(strs) => {
                  if strs.get_str() != "string" {
                    panic!();
                  }
                }
                _ => {
                  panic!();
                }
              }
            }
            _ => {
              panic!()
            }
          }
        }
      }

      Err(e) => {
        panic!(e);
      }
    }
  }

  #[test]
  fn call() {
    let mut lex = lexers::lex("let a = a(a, a) + 1;");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Var(var) => {
              if var.get_name() != "a" {
                panic!();
              }

              match var.get_node_index(0) {
                ast::ast::Syntax::Call(call) => {
                  if call.get_name() != "a" {
                    panic!();
                  }

                  for call in call.get_argment().iter() {
                    match call {
                      ast::ast::Syntax::Var(var) => {
                        if var.get_name() != "a" {
                          panic!();
                        }
                      }

                      _ => {
                        panic!();
                      }
                    }
                  }

                  match call.get_node_index(0) {
                    ast::ast::Syntax::Bin(bin) => {
                      if bin.get_bin() != '+' {
                        panic!();
                      }

                      match bin.get_node_index(0) {
                        ast::ast::Syntax::Num(num) => {
                          if num.get_num() != 1 {
                            panic!();
                          }
                        }
                        _ => {
                          panic!();
                        }
                      }
                    }

                    _ => {
                      panic!();
                    }
                  }
                }
                _ => {
                  panic!();
                }
              }
            }

            _ => {
              panic!();
            }
          }
        }
      }

      Err(e) => {
        panic!(e);
      }
    }
  }

  #[test]
  fn consts() {
    let mut lex = lexers::lex("const a = 1;");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Var(var) => {
              if var.get_is_mutable() != false {
                panic!();
              }
            }

            _ => {
              panic!();
            }
          }
        }
      }

      Err(e) => {
        panic!(e);
      }
    }
  }

  #[test]
  fn lets() {
    let mut lex = lexers::lex("let a = 1;");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Var(var) => {
              if var.get_is_mutable() != true {
                panic!();
              }
            }

            _ => {
              panic!();
            }
          }
        }
      }

      Err(e) => {
        panic!(e);
      }
    }
  }
}
