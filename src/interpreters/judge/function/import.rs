use std::fs::File;
use std::io::prelude::*;

use super::super::super::interpreter::Interpreter;
use crate::error::result;

use crate::lexer::lexers;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax};
use crate::parser::parsers;

use std::path::Path;

impl Interpreter {
  pub(crate) fn import(&self, path: &str) -> Result<ast::VariableAST, result::Error> {
    let my_path = Path::new(self.get_path());
    let parent = my_path.parent();
    let join_path = parent
      .ok_or(result::Error::InterpreterError(format!(
        "{} is not found",
        my_path.display()
      )))?
      .join(path);

    let mut f: File;
    match File::open(join_path) {
      Ok(file) => {
        f = file;
      }

      Err(e) => {
        return Err(result::Error::FileReadError(e.to_string()));
      }
    }

    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
      Ok(_) => {}
      Err(e) => {
        return Err(result::Error::FileReadError(e.to_string()));
      }
    }

    let mut lex = lexers::lex(&contents);
    let result = lex.run().get_tokens();
    println!("{:?}", result);

    let mut parse = parsers::Parsers::new(result.to_vec());
    let result = parse.run()?;
    println!("{:?}", result);

    let name = Path::new(path);
    let mut var_ast =
      ast::VariableAST::new(name.file_stem().unwrap().to_str().unwrap(), false, true);

    for ast in result.get_node().iter() {
      match ast {
        Syntax::Fn(fun) => {
          var_ast.push_function(fun.clone());
        }

        Syntax::Var(var) => {
          var_ast.push_variable(var.clone());
        }

        Syntax::Import(import) => {
          let inner = import
            .get_node_index(0)
            .ok_or(result::Error::InterpreterError(format!("import error")))?;
          match inner {
            Syntax::Str(strs) => {
              var_ast.push_variable(self.import(strs.get_str())?);
            }

            _ => {
              return Err(result::Error::InterpreterError(
                "please specify import as a string ".to_string(),
              ));
            }
          }
        }

        _ => {
          return Err(result::Error::InterpreterError(
            "the syntax is not written inside the function".to_string(),
          ));
        }
      }
    }

    return Ok(var_ast);
  }
}
