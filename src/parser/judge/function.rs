use super::super::super::lexer::token;
use super::super::ast::{ast, ast::Node, ast::Syntax, ast::Type};
use super::super::parsers::Parsers;
use crate::error::result;

static TOKEN: token::Token = token::Token::new();

impl Parsers {
    pub(crate) fn fucntion(&mut self) -> Result<ast::Syntax, result::Error> {
        let mut fn_ast: ast::FunctionAST;
        self.index_inc();

        let judge = self.judge().ok_or(result::Error::SyntaxError(
            "fucntion name error possible parser bug".to_string(),
        ))?;
        match judge? {
            Syntax::Var(var) => {
                if var.get_node_len() < 1 {
                    fn_ast = ast::FunctionAST::new(var.get_name());
                } else {
                    return Err(result::Error::SyntaxError(
                        "fucntion name error possible parser bug".to_string(),
                    ));
                }
            }
            _ => {
                return Err(result::Error::SyntaxError(
                    "fucntion name error possible parser bug".to_string(),
                ));
            }
        }

        self.index_inc();
        let paren_left_token = self
            .get_tokens(self.get_index())
            .ok_or(result::Error::SyntaxError(
                "out of index fucntion name error possible parser bug".to_string(),
            ))?
            .get_token();
        if paren_left_token != TOKEN._paren_left {
            return Err(result::Error::SyntaxError("( not enough".to_string()));
        }

        loop {
            self.index_inc();

            let paren_right_token = self
                .get_tokens(self.get_index())
                .ok_or(result::Error::SyntaxError(
                    "out of index fucntion param error possible parser bug".to_string(),
                ))?
                .get_token();

            let verification_token = self
                .get_tokens(self.get_index() + 1)
                .ok_or(result::Error::SyntaxError(
                    "out of index fucntion param error possible parser bug".to_string(),
                ))?
                .get_token();

            if paren_right_token == TOKEN._paren_right {
                break;
            }

            let judge = self.judge().ok_or(result::Error::SyntaxError(format!(
                "functin {} param error",
                fn_ast.get_name()
            )))?;

            match judge? {
                Syntax::Var(mut var) => {
                    if verification_token != TOKEN._colon {
                        return Err(result::Error::SyntaxError(format!(
                            "fucntion {} param type error",
                            fn_ast.get_name()
                        )));
                    }

                    var.set_type(self.check_types()?);
                    fn_ast.push_param(Syntax::Var(var));
                    self.index_inc();

                    match self.get_tokens(self.get_index()) {
                        Some(tokens) => {
                            if tokens.get_token() == TOKEN._paren_right {
                                break;
                            } else if tokens.get_token() == TOKEN._comma {
                                continue;
                            } else {
                                return Err(result::Error::SyntaxError(format!(
                                    "function ) or , not found {}",
                                    fn_ast.get_name()
                                )));
                            }
                        }

                        None => {
                            return Err(result::Error::SyntaxError(format!(
                                "function ) not found {}",
                                fn_ast.get_name()
                            )));
                        }
                    }
                }

                _ => {
                    return Err(result::Error::SyntaxError(format!(
                        "fucntion {} param type error",
                        fn_ast.get_name()
                    )));
                }
            }
        }

        match self.check_types() {
            Ok(types) => fn_ast.set_type(types),
            Err(_) => {
                //返り値がなくても良いため
            }
        }

        self.index_inc();

        let judge = self
            .judge()
            .ok_or(result::Error::SyntaxError(format!("fucntion scope error")))??;
        match judge {
            ast::Syntax::Scope(_) => fn_ast.push_node(judge),
            _ => {
                return Err(result::Error::SyntaxError(format!(
                    "{} is scope not found",
                    fn_ast.get_name()
                )))
            }
        }
        return Ok(ast::Syntax::Fn(fn_ast));
    }
}
