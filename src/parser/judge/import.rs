use super::super::ast::ast;
use super::super::ast::ast::{Node, Syntax};
use super::super::parsers::Parsers;
use crate::error::result;

impl Parsers {
    pub(crate) fn import(&mut self) -> Result<Syntax, result::Error> {
        self.index_inc();
        let judge = self
            .judge()
            .ok_or(result::Error::SyntaxError(format!("import error")))?;
        match judge? {
            Syntax::Str(strs) => {
                let mut import_ast = ast::ImportAST::new();
                import_ast.push_node(Syntax::Str(strs));
                return Ok(Syntax::Import(Box::new(import_ast)));
            }

            _ => {
                return Err(result::Error::SyntaxError(
                    "please specify import as a string ".to_string(),
                ));
            }
        }
    }
}
