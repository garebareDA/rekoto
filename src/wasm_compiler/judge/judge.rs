use crate::error::result;
use crate::parser::ast::ast::Syntax;

use super::super::compiler::Compiler;

impl Compiler {
    pub fn judge(&mut self, ast: &Syntax) -> Result<(), result::Error> {
        match ast {
            Syntax::Bin(bin) => {
                return Err(result::Error::CompileError(format!(
                    "{} binary error",
                    bin.get_bin()
                )));
            }

            _ => {
                return Err(result::Error::CompileError("Not implemented.".into()));
            }
        }
    }
}
