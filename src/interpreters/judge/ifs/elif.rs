use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::Syntax;

impl Interpreter {
    pub(crate) fn elif(
        &mut self,
        elif: &Box<ast::ElifAST>,
    ) -> (
        Option<Result<Option<Syntax>, result::Error>>,
        Option<String>,
    ) {
        match self.else_check("elif") {
            Ok(o) => match o {
                Some(()) => {}
                None => return (None, None),
            },

            Err(e) => {
                return (Some(Err(e)), None);
            }
        }

        match self.formula(elif.get_judge()) {
            Ok(bools) => {
                return self.ifs_judge(bools, elif, "elif");
            }
            Err(e) => (Some(Err(e)), None),
        }
    }
}
