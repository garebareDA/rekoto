use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax, Type, Types};

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
                    self.member_type_check(j, i)?;
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

    fn member_type_check(
        &mut self,
        member: &ast::MemberAST,
        serched_member: &ast::MemberAST,
    ) -> Result<(), result::Error> {
        let serched_member_type =
            serched_member
                .get_type()
                .ok_or(result::Error::InterpreterError(
                    "member type not found".to_string(),
                ))?;

        let member_node = member
            .get_node_index(0)
            .ok_or(result::Error::InterpreterError(format!(
                "{} member error",
                member.get_name()
            )))?;

        match self.formula(member_node)? {
            Syntax::Num(_) => match serched_member_type {
                Types::Number => return Ok(()),
                _ => {}
            },
            Syntax::Str(_) => match serched_member_type {
                Types::String => return Ok(()),
                _ => {}
            },
            Syntax::Bool(_) => match serched_member_type {
                Types::Bool => return Ok(()),
                _ => {}
            },
            _ => {}
        }

        return Err(result::Error::InterpreterError(
            "memeber miss matched type".to_string(),
        ));
    }
}
