use super::super::super::interpreter::Interpreter;
use crate::parser::ast::ast::{CallAST, Node, Syntax};

impl Interpreter{
  pub(crate) fn call(&self, call:&CallAST) -> Result<(), String>{
    let node_len = call.get_node_len();
    let argment_len = call.get_argment_len();
    let argment = call.get_argment();
    let name = call.get_name();

    if name == "print" {
      if node_len != 0 {
        return Err("error print cannot be incorporated into formulas".to_string());
      }

      if argment_len != 1 {
        return Err("error print argment 1".to_string());
      }

      match argment.get(0){
        Some(argment) => {
          match argment {
            Syntax::Str(strs) => {
              println!("{}", strs.get_str());
            }

            Syntax::Num(num) => {
              println!("{}", num.get_num());
            }

            _ => {}
          }
        }
        None => {
          return Err("error print".to_string());
        }
      }

      return Ok(());
    }

    return Err(format!("not found function {}", name));
  }
}