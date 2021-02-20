pub mod parsers;
pub mod ast;
pub mod judge;


#[cfg(test)]
mod tests {
  use super::super::lexer::lexers;
  use super::super::parser::{ast, parsers};
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
                      if bin.get_bin() != "+" {
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
                      if bin.get_bin() != "+" {
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

  #[test]
  fn scope() {
    let mut lex = lexers::lex("{{let a = 1;}}\n{let a = 1;{}}");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        let obj = result.get_node();
        match &obj[0] {
          ast::ast::Syntax::Scope(scope) => {
            let scope = scope.get_scope();
            match &scope[0] {
              ast::ast::Syntax::Scope(_) => {}
              _ => {
                panic!();
              }
            }
          }

          _ => {
            panic!();
          }
        }

        match &obj[1] {
          ast::ast::Syntax::Scope(scope) => {
            let scope = scope.get_scope();
            match &scope[1] {
              ast::ast::Syntax::Scope(_) => {}
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

      Err(e) => {
        panic!(e);
      }
    }
  }

  #[test]
  fn ifs() {
    let mut lex = lexers::lex("if 1 < 0 {};");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Ifs(ifs) => {
              let judge = ifs.get_judge();
              match &judge {
                ast::ast::Syntax::Num(num) => {
                  match num.get_node_index(0) {
                    ast::ast::Syntax::Bin(bin) => {
                      if bin.get_bin() != "<" {
                        panic!();
                      }

                      match bin.get_node_index(0) {
                        ast::ast::Syntax::Num(_) => {}
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
                _ => {
                  panic!();
                }
              }

              println!("{:?}", ifs.get_scope());
              match ifs.get_scope()[0] {
                ast::ast::Syntax::Scope(_) => {}
                _ => {
                  panic!()
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
}