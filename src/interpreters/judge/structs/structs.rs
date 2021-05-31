use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax};

impl Interpreter {
  pub fn structs(&mut self, structs: &ast::StructAST) -> Result<(), result::Error> {
    let name = structs.get_name();
    let serched_struct = self
      .serch_struct(name)
      .ok_or(result::Error::InterpreterError(format!(
        "{} struct not found",
        name
      )))?;

    if structs.get_member_len() != serched_struct.get_member_len() {
      return Err(result::Error::InterpreterError(format!(
        "{} don't have the right number of members",
        name
      )));
    }

    for i in serched_struct.get_member().iter() {
      let mut member_flag = false;
      let mut member_name = "";

      for j in structs.get_member().iter() {
        if j.get_name() == member_name {
          return Err(result::Error::InterpreterError(format!(
            "{} don't have the right number of members",
            name
          )));
        }

        if j.get_name() == i.get_name() {
          member_flag = true;
        }



        member_name = j.get_name();
      }

      if member_flag == false {
        return Err(result::Error::InterpreterError(format!(
          "{} member not found",
          i.get_name()
        )));
      }
    }

    return Ok(());
  }

  fn member_type_check(&mut self, member: &ast::MemberAST, serched_member: &ast::MemberAST) {
    
  }
}
