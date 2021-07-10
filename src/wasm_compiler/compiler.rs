use crate::error::result;
use crate::interpreters::{functions, variables, variables::Scope};
use crate::parser::ast;
use crate::parser::ast::ast::Node;
use super::emitter;
use std::fmt::Binary;
pub struct Compiler {
    var: variables::Variables,
    fun: functions::Functions,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            var: variables::Variables::new(),
            fun: functions::Functions::new(),
        }
    }

    pub fn compile(&mut self) -> Result<Vec<u8>, result::Error> {
        let emitter = emitter::emiter();
        for e in emitter {
            println!("{:018b}", &e);
        }
        return Ok(emitter);
    }

    pub(crate) fn push_scope(&mut self) {
        self.var.push_scope();
        self.fun.push_scope();
    }

    pub(crate) fn pop_scope(&mut self) {
        self.var.pop_scope();
        self.fun.pop_scope();
    }

    fn serch_fun(&self, name: impl Into<String>) -> Option<ast::ast::FunctionAST> {
        self.fun.serch(&name.into())
    }
}
