use crate::error::result;
use crate::interpreters::{functions, variables, variables::Scope};
use crate::parser::ast;
use crate::parser::ast::ast::Node;

pub struct Compiler {
    path: String,
    name: String,
    var: variables::Variables,
    fun: functions::Functions,
}

impl Compiler {
    pub fn new(path: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            var: variables::Variables::new(),
            fun: functions::Functions::new(),
        }
    }

    pub fn compile(&mut self) -> Result<(), result::Error> {
        self.push_scope();
        let main = self
            .serch_fun("main")
            .ok_or(result::Error::CompileError("not found main fucntion".into()))?;
        for ast in main.get_node().iter() {
            
        }
        return Ok(());
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
