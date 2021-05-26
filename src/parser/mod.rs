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
    let mut lex = lexers::lex("let a = a + 1;");
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    match result {
      Ok(result) => {
        for obj in result.get_node().iter() {
          match obj {
            ast::ast::Syntax::Var(var) => {
              assert_eq!(var.get_name(), "a");

              match var.get_node_index(0).unwrap() {
                ast::ast::Syntax::Var(var) => {
                  assert_eq!(var.get_name(), "a");

                  match var.get_node_index(0).unwrap() {
                    ast::ast::Syntax::Bin(bin) => {
                      assert_eq!(bin.get_bin(), "+");

                      match bin.get_node_index(0).unwrap() {
                        ast::ast::Syntax::Num(num) => {
                          assert_eq!(num.get_num(), 1)
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
              assert_eq!(var.get_name(), "a");

              match var.get_node_index(0).unwrap() {
                ast::ast::Syntax::Str(strs) => {
                  assert_eq!(strs.get_str(), "string")
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
              assert_eq!(var.get_name(), "a");

              match var.get_node_index(0).unwrap() {
                ast::ast::Syntax::Call(call) => {
                  assert_eq!(call.get_name(), "a");

                  for call in call.get_argment() {
                    match call {
                      ast::ast::Syntax::Var(var) => {
                        assert_eq!(var.get_name(), "a");
                      }

                      _ => {
                        panic!();
                      }
                    }
                  }

                  match call.get_node_index(0).unwrap() {
                    ast::ast::Syntax::Bin(bin) => {
                      assert_eq!(bin.get_bin(), "+");

                      match bin.get_node_index(0).unwrap() {
                        ast::ast::Syntax::Num(num) => {
                          assert_eq!(num.get_num(), 1);
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
              assert_eq!(var.get_is_mutable(), false);
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
              assert_eq!(var.get_is_mutable(), true);
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
    let mut lex = lexers::lex("if 1 < 0 {print('hello')}");
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
                ast::ast::Syntax::Num(num) => match num.get_node_index(0).unwrap() {
                  ast::ast::Syntax::Bin(bin) => {
                    assert_eq!(bin.get_bin(), "<");
                    match bin.get_node_index(0).unwrap() {
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
    let mut lex = lexers::lex("elif 1 < 0 {print('hello')}");
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
                ast::ast::Syntax::Num(num) => match num.get_node_index(0).unwrap() {
                  ast::ast::Syntax::Bin(bin) => {
                    assert_eq!(bin.get_bin(), "<");

                    match bin.get_node_index(0).unwrap() {
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
    let mut lex = lexers::lex("else {print('hello')}");
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

              let add = fors.get_counter();
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
              assert_eq!(fnc.get_name(), "a");
              match fnc.get_type() {
                Some(t) => if t != &ast::ast::Types::Number {},

                None => {
                  panic!();
                }
              }

              for param in fnc.get_param().iter() {
                match param {
                  ast::ast::Syntax::Var(var) => {
                    assert_eq!(var.get_name(), "a");
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
                assert_eq!(num.get_num(), 1);
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
                assert_eq!(t, &ast::ast::Types::String);
                assert_eq!(var.get_name(), "a");

                match var.get_node_index(0).unwrap() {
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

  #[test]
  fn boolean() {
    let mut lex = lexers::lex("let a:bool = true;");
    let result = lex.run().get_tokens();

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Var(var) => {
              match var.get_type() {
                Some(t) => match t {
                  ast::ast::Types::Bool => {}

                  _ => {
                    panic!();
                  }
                },
                None => {
                  panic!();
                }
              }

              match var.get_node_index(0).unwrap() {
                ast::ast::Syntax::Bool(bools) => {
                  assert_ne!(bools.get_bool(), false);
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
  fn import() {
    let mut lex = lexers::lex("import './url/test';");
    let result = lex.run().get_tokens();

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Import(import) => match import.get_node_index(0) {
              Some(inner) => match inner {
                ast::ast::Syntax::Str(strs) => {
                  assert_eq!(strs.get_str(), "./url/test");
                }

                _ => {
                  panic!();
                }
              },
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

  #[test]
  fn structs() {
    let mut lex = lexers::lex(
      "
    struct test {
      a: string,
      b: number,
    }
    ",
    );
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();

    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Struct(st) => {
              match st.get_member_index(0) {
                Some(inner) => {
                  assert_eq!(inner.get_name(), "a");
                  match inner.get_type().clone().unwrap() {
                    ast::ast::Types::String => {}
                    _ => {
                      panic!();
                    }
                  }
                }
                None => {
                  panic!();
                }
              }

              match st.get_member_index(1) {
                Some(inner) => {
                  assert_eq!(inner.get_name(), "b");
                  match inner.get_type().clone().unwrap() {
                    ast::ast::Types::Number => {}
                    _ => {
                      panic!();
                    }
                  }
                }
                None => {
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
  fn instance() {
    let mut lex = lexers::lex(
      "
    test {
      a: 1,
      b: 1,
    }
    ",
    );
    let result = lex.run().get_tokens();
    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run();
    match result {
      Ok(result) => {
        for obj in result.get_node() {
          match obj {
            ast::ast::Syntax::Struct(structs) => {
              match structs.get_member_index(0) {
                Some(mem) => match mem.get_node_index(0) {
                  Some(syn) => match syn {
                    ast::ast::Syntax::Num(num) => {
                      assert_eq!(1, num.get_num());
                    }
                    _ => {
                      panic!()
                    }
                  },
                  None => {
                    panic!()
                  }
                },
                None => {
                  panic!();
                }
              }

              match structs.get_member_index(1) {
                Some(mem) => match mem.get_node_index(0) {
                  Some(syn) => match syn {
                    ast::ast::Syntax::Num(num) => {
                      assert_eq!(1, num.get_num());
                    }
                    _ => {
                      panic!()
                    }
                  },
                  None => {
                    panic!()
                  }
                },
                None => {
                  panic!();
                }
              }
            }
            _ => panic!(),
          }
        }
      }
      Err(e) => {
        panic!(e)
      }
    }
  }
}
