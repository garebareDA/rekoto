pub mod ast;
pub mod judge;
pub mod parsers;

#[cfg(test)]
mod tests {
  use crate::lexer::lexers;
  use crate::parser::ast::ast::{Node, Type};
  use crate::parser::{ast, parsers};

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
            let scope = scope.get_node();
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
            let scope = scope.get_node();
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
    let mut lex = lexers::lex("if 1 < 0 {}");
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
                ast::ast::Syntax::Num(num) => match num.get_node_index(0) {
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
                },
                _ => {
                  panic!();
                }
              }

              match ifs.get_node()[0] {
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

  #[test]
  fn elif() {
    let mut lex = lexers::lex("elif 1 < 0 {}");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Elif(ifs) => {
              let judge = ifs.get_judge();
              match &judge {
                ast::ast::Syntax::Num(num) => match num.get_node_index(0) {
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
                },
                _ => {
                  panic!();
                }
              }

              match ifs.get_node()[0] {
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

  #[test]
  fn elses() {
    let mut lex = lexers::lex("else {}");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Else(ifs) => match ifs.get_node()[0] {
              ast::ast::Syntax::Scope(_) => {}
              _ => {
                panic!()
              }
            },

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
  fn fors() {
    let mut lex = lexers::lex("for let i = 0; i < 5; i++; {}");
    let result = lex.run().get_tokens();

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::For(fors) => {
              let init = fors.get_init();
              match init {
                ast::ast::Syntax::Var(_) => {}
                _ => {
                  panic!();
                }
              }

              let judge = fors.get_judge();
              match judge {
                ast::ast::Syntax::Var(_) => {}
                _ => {
                  panic!()
                }
              }

              let add = fors.get_add();
              match add {
                ast::ast::Syntax::Var(_) => {}
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

  #[test]
  fn function() {
    let mut lex = lexers::lex("fn a(a:number, a:number):number {}");
    let result = lex.run().get_tokens();

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Fn(fnc) => {
              if fnc.get_name() != "a" {
                panic!();
              }

              match fnc.get_type() {
                Some(t) => if t != &ast::ast::Types::Number {},

                None => {
                  panic!();
                }
              }

              for param in fnc.get_param().iter() {
                match param {
                  ast::ast::Syntax::Var(var) => {
                    if var.get_name() != "a" {
                      panic!();
                    }

                    match var.get_type() {
                      Some(t) => {
                        if t != &ast::ast::Types::Number {
                          panic!()
                        }
                      }

                      None => {
                        panic!()
                      }
                    }
                  }

                  _ => {
                    panic!();
                  }
                }
              }

              match &fnc.get_node()[0] {
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
      }

      Err(e) => {
        panic!(e);
      }
    }
  }

  #[test]
  fn returns() {
    let mut lex = lexers::lex("return 1 + 1;");
    let result = lex.run().get_tokens();

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Return(ret) => match &ret.get_node()[0] {
              ast::ast::Syntax::Num(num) => {
                if 1 != num.get_num() {
                  panic!();
                }
              }

              _ => {
                panic!();
              }
            },
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
  fn types() {
    let mut lex = lexers::lex("let a:string = \"string\";");
    let result = lex.run().get_tokens();

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Var(var) => match var.get_type() {
              Some(t) => {
                if t != &ast::ast::Types::String {
                  panic!();
                }

                if var.get_name() != "a" {
                  panic!();
                }

                match var.get_node()[0] {
                  ast::ast::Syntax::Str(_) => {}
                  _ => {
                    panic!()
                  }
                }
              }

              None => {
                panic!();
              }
            },
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
