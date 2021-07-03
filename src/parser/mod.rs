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
        let fun = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "let a = a + 1;";
        test_run(syn, fun);
    }

    #[test]
    fn strings() {
        let fun = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "let a = \"string\";";
        test_run(syn, fun);
    }

    #[test]
    fn call() {
        let fun = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "let a = a(a, a) + 1;";
        test_run(syn, fun);
    }

    #[test]
    fn consts() {
        let fun = |obj: &ast::ast::Syntax| match obj {
            ast::ast::Syntax::Var(var) => {
                assert_eq!(var.get_is_mutable(), false);
            }

            _ => {
                panic!();
            }
        };
        let syn = "const a = 1;";
        test_run(syn, fun);
    }

    #[test]
    fn lets() {
        let fun = |obj: &ast::ast::Syntax| match obj {
            ast::ast::Syntax::Var(var) => {
                assert_eq!(var.get_is_mutable(), true);
            }

            _ => {
                panic!();
            }
        };
        let syn = "let a = 1;";
        test_run(syn, fun);
    }

    #[test]
    fn scope() {
        let fun = |obj: &ast::ast::Syntax| match obj {
            ast::ast::Syntax::Scope(_) => {}

            _ => {
                panic!();
            }
        };
        let syn = "{{let a = 1;}}\n{let a = 1;{}}";
        test_run(syn, fun);
    }

    #[test]
    fn ifs() {
        let fun = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "if 1 < 0 {print('hello')}";
        test_run(syn, fun);
    }

    #[test]
    fn elif() {
        let fun = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "elif 1 < 0 {print('hello')}";
        test_run(syn, fun);
    }

    #[test]
    fn elses() {
        let func = |obj: &ast::ast::Syntax| match obj {
            ast::ast::Syntax::Else(ifs) => match ifs.get_node()[0] {
                ast::ast::Syntax::Scope(_) => {}
                _ => {
                    panic!()
                }
            },

            _ => {
                panic!();
            }
        };
        let syn = "else {print('hello')}";
        test_run(syn, func);
    }

    #[test]
    fn fors() {
        let func = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "for let i = 0; i < 5; i++; {}";
        test_run(syn, func);
    }

    #[test]
    fn function() {
        let func = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "fn a(a:number, a:number):number {}";
        test_run(syn, func);
    }

    #[test]
    fn returns() {
        let func = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "return 1 + 1;";
        test_run(syn, func);
    }

    #[test]
    fn types() {
        let func = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "let a:string = \"string\";";
        test_run(syn, func);
    }

    #[test]
    fn boolean() {
        let fun = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "let a:bool = true;";
        test_run(syn, fun);
    }

    #[test]
    fn import() {
        let fun = |obj: &ast::ast::Syntax| match obj {
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
        };
        let syn = "import './url/test';";
        test_run(syn, fun);
    }

    #[test]
    fn structs() {
        let fun = |obj: &ast::ast::Syntax| match obj {
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
        };

        let syn = "
    struct test {
      a: string,
      b: number,
    }
    ";

        test_run(syn, fun);
    }

    #[test]
    fn instance() {
        let fun = |obj: &ast::ast::Syntax| match obj {
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
        };

        let syn = "
    test {
      a: 1,
      b: 1,
    }
    ";

        test_run(syn, fun);
    }

    fn test_run<F: Fn(&ast::ast::Syntax)>(syn: &str, test: F) {
        let mut lex = lexers::lex(syn);
        let result = lex.run().get_tokens();
        let mut parse = parsers::Parsers::new(result.to_vec());
        let result = parse.run();

        match result {
            Ok(result) => {
                for obj in result.get_node() {
                    test(obj);
                }
            }

            Err(e) => {
                panic!(e);
            }
        }
    }
}
