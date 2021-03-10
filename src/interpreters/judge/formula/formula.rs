use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::lexer::token;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax};
static TOKEN: token::Token = token::Token::new();

#[derive(Debug, Clone)]
pub enum FormulaType {
  Bool(bool),
  Strings(String),
  Number(i64),
}

#[derive(Debug, Clone)]
pub struct Formula {
  bin_stack: Vec<i64>,
  stack: Vec<FormulaType>,
}

impl Formula {
  pub fn new() -> Self {
    Self {
      bin_stack: Vec::new(),
      stack: Vec::new(),
    }
  }

  pub fn run(&mut self) -> Result<&FormulaType, result::Error> {
    let mut index = 0;
    loop {
      let mut i = 0;
      if self.stack.len() <= 1 {
        break;
      }

      'inner: loop {
        if self.bin_stack.len() <= i {
          break 'inner;
        }

        let bin: i64;
        match self.bin_stack.get(i) {
          Some(b) => bin = *b,
          None => return Err(result::Error::InterpreterError(format!("calclation error"))),
        }

        if index == 0 {
          //単行演算子
        }

        if index == 1 {
          // * / %
          if bin == TOKEN._div {
            let both_side = self.both_side(i)?;
            let result = self.div(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }

          if bin == TOKEN._mul {
            let both_side = self.both_side(i)?;
            let result = self.mul(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }

          if bin == TOKEN._sur {
            let both_side = self.both_side(i)?;
            let result = self.sur(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }
        }

        if index == 2 {
          // + -
          if bin == TOKEN._add {
            let both_side = self.both_side(i)?;
            let result = self.add(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }

          if bin == TOKEN._sub {
            let both_side = self.both_side(i)?;
            let result = self.sub(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }
        }

        if index == 3 {
          // >= <= < >
          if bin == TOKEN._greater_equ {
            let both_side = self.both_side(i)?;
            let result = self.greater_equ(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }

          if bin == TOKEN._less_equ {
            let both_side = self.both_side(i)?;
            let result = self.less_equ(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }

          if bin == TOKEN._less {
            let both_side = self.both_side(i)?;
            let result = self.less(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }

          if bin == TOKEN._greater {
            let both_side = self.both_side(i)?;
            let result = self.grater(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }
        }

        if index == 4 {
          // == !=
          if bin == TOKEN._equ {
            let both_side = self.both_side(i)?;
            let result = self.equal(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }

          if bin == TOKEN._not_equ {
            let both_side = self.both_side(i)?;
            let result = self.not_equal(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }
        }

        if index == 5 {
          // &&
          if bin == TOKEN._and {
            let both_side = self.both_side(i)?;
            let result = self.and(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }
        }

        if index == 6 {
          // ||
          if bin == TOKEN._or {
            let both_side = self.both_side(i)?;
            let result = self.or(both_side.0, both_side.1)?;
            self.insert_stack(&mut i, result);
            continue;
          }
        }

        i += 1;
      }
      index += 1;
    }

    if self.bin_stack.len() != 0 {
      return Err(result::Error::InterpreterError(
        "missing or many operators or not present".to_string(),
      ));
    }

    if self.stack.len() == 1 {
      match self.stack.get(0) {
        Some(stack) => Ok(stack),
        None => {
          return Err(result::Error::InterpreterError(format!(
            "calclation error intepreter bug"
          )))
        }
      }
    } else {
      return Err(result::Error::InterpreterError(format!(
        "missing or many numbers or not present"
      )));
    }
  }

  fn both_side(&mut self, i: usize) -> Result<(FormulaType, FormulaType), result::Error> {
    if self.stack.len() < i + 1 {
      return Err(result::Error::InterpreterError("temp error".to_string()));
    }
    let left = self.stack.remove(i);
    let right = self.stack.remove(i);
    return Ok((left, right));
  }

  pub fn push_bin(&mut self, bin: i64) {
    self.bin_stack.push(bin);
  }

  pub fn push_stack(&mut self, stack: FormulaType) {
    self.stack.push(stack);
  }

  pub fn pop_bin(&mut self, index: usize) -> Result<i64, result::Error> {
    if self.bin_stack.len() > index {
      return Err(result::Error::InterpreterError(
        "pop bin error interpreter bug".to_string(),
      ));
    }

    let bin = self.bin_stack.remove(index);
    return Ok(bin);
  }

  pub fn pop_stack(&mut self, index: usize) -> Result<FormulaType, result::Error> {
    if self.stack.len() > index {
      return Err(result::Error::InterpreterError(
        "pop stack error interpreter bug".to_string(),
      ));
    }

    let formula = self.stack.remove(index);
    return Ok(formula);
  }

  fn insert_stack(&mut self, index: &mut usize, result: FormulaType) {
    self.stack.insert(*index, result);
    self.bin_stack.remove(*index);
    if 0 < *index {
      *index = *index - 1;
    }
  }
}

impl Interpreter {
  pub(crate) fn formula(&mut self, formula: &ast::Syntax) -> Result<Syntax, result::Error> {
    let mut formulas = Formula::new();
    self.formula_push(&mut formulas, formula)?;
    match formulas.run() {
      Ok(result) => match result {
        FormulaType::Number(num) => {
          return Ok(Syntax::Num(ast::NumberAST::new(*num)));
        }

        FormulaType::Strings(strs) => {
          return Ok(Syntax::Str(ast::StringAST::new(strs)));
        }

        FormulaType::Bool(bools) => {
          return Ok(Syntax::Bool(ast::BoolAST::new(*bools)));
        }
      },

      Err(e) => {
        return Err(e);
      }
    }
  }

  fn formula_push(&self, formula: &mut Formula, ast: &Syntax) -> Result<(), result::Error> {
    match ast {
      Syntax::Bin(bin) => {
        formula.push_bin(bin.get_token());
        return self.formula_continue(bin, formula);
      }
      Syntax::Bool(bools) => {
        formula.push_stack(FormulaType::Bool(bools.get_bool()));
        return self.formula_continue(bools, formula);
      }
      Syntax::Num(num) => {
        formula.push_stack(FormulaType::Number(num.get_num()));
        return self.formula_continue(num, formula);
      }
      Syntax::Str(strs) => {
        formula.push_stack(FormulaType::Strings(strs.get_str().into()));
        return self.formula_continue(strs, formula);
      }
      Syntax::Var(vars) => match self.serch_var(vars.get_name()) {
        Some(inner) => {
          match self.formula_push(formula, inner) {
            Ok(_) => {}
            Err(e) => {
              return Err(e);
            }
          }
          return self.formula_continue(vars, formula);
        }

        None => {
          return Err(result::Error::InterpreterError(format!(
            "{} not a var",
            vars.get_name()
          )))
        }
      },
      //TODO callの実装
      _ => Err(result::Error::InterpreterError(
        "variable error cannot be assigned".to_string(),
      )),
    }
  }

  fn formula_continue<T: Node>(
    &self,
    node: &T,
    formula: &mut Formula,
  ) -> Result<(), result::Error> {
    match node.get_node_index(0) {
      Some(ast) => self.formula_push(formula, ast),
      None => {
        return Ok(());
      }
    }
  }
}
