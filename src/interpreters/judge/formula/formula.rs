use super::super::super::interpreter::Interpreter;
use crate::error::result;
use crate::parser::ast::ast;
use crate::parser::ast::ast::{Node, Syntax};

pub enum FormulaType {
  Bool(bool),
  Strings(String),
  Number(i64),
}

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
}

impl Interpreter {
  pub(crate) fn formula(&mut self, formula: &ast::Syntax) -> Result<Syntax, result::Error> {
    let mut formulas = Formula::new();
    match self.formula_push(&mut formulas, formula) {
      Ok(_) => {}
      Err(e) => return Err(e),
    }

    return Err(result::Error::InterpreterError(
      "formula error intepreter bug".to_string(),
    ));
  }

  fn formula_push(
    &self,
    formula: &mut Formula,
    ast: &ast::Syntax,
  ) -> Result<(), result::Error> {
    return self.formula_check(formula, ast);
  }

  fn formula_check(&self, formula: &mut Formula, ast: &Syntax) -> Result<(), result::Error> {
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
          return self.formula_check(formula, inner);
        }

        None => {
          return Err(result::Error::InterpreterError(format!(
            "{} not a var",
            vars.get_name()
          )))
        }
      },

      _ => Err(result::Error::InterpreterError("".to_string())),
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
