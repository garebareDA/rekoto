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
  Var(ast::VariableAST),
  Call(ast::CallAST),
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

        let bin = *self
          .bin_stack
          .get(i)
          .ok_or(result::Error::InterpreterError(format!("calclation error")))?;
        if index == 0 {
          //単行演算子
          if bin == TOKEN._dot {
            return Err(result::Error::InterpreterError(format!(
              "caluculation error dot oprator"
            )));
          }

          if bin == TOKEN._inc {
            return Err(result::Error::InterpreterError(format!(
              "caluculation error inclement oprator"
            )));
          }

          if bin == TOKEN._dec {
            return Err(result::Error::InterpreterError(format!(
              "caluculation error declement oprator"
            )));
          }
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
      return Ok(
        self
          .stack
          .get(0)
          .ok_or(result::Error::InterpreterError(format!(
            "calclation error intepreter bug"
          ))),
      )?;
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

#[derive(PartialEq)]
enum FormulaBeforeState {
  Bin,
  Nega,
  None,
}

impl Interpreter {
  pub(crate) fn formula(&mut self, formula: &ast::Syntax) -> Result<Syntax, result::Error> {
    let mut formulas = Formula::new();
    self.formula_push(&mut formulas, formula, FormulaBeforeState::None)?;
    match formulas.run()? {
      FormulaType::Number(num) => {
        return Ok(Syntax::Num(ast::NumberAST::new(*num)));
      }

      FormulaType::Strings(strs) => {
        return Ok(Syntax::Str(ast::StringAST::new(strs)));
      }

      FormulaType::Bool(bools) => {
        return Ok(Syntax::Bool(ast::BoolAST::new(*bools)));
      }

      _ => {
        return Err(result::Error::InterpreterError(format!(
          "formula return error possible interpreter error"
        )))
      }
    }
  }

  fn formula_push(
    &mut self,
    formula: &mut Formula,
    ast: &Syntax,
    state: FormulaBeforeState,
  ) -> Result<(), result::Error> {
    match ast {
      Syntax::Bin(bin) => {
        if bin.get_token() == TOKEN._sub && state == FormulaBeforeState::Bin {
          return self.formula_continue(bin, formula, FormulaBeforeState::Nega);
        }

        formula.push_bin(bin.get_token());
        return self.formula_continue(bin, formula, FormulaBeforeState::Bin);
      }
      Syntax::Bool(bools) => {
        formula.push_stack(FormulaType::Bool(bools.get_bool()));
        return self.formula_continue(bools, formula, FormulaBeforeState::None);
      }
      Syntax::Num(num) => {
        let mut result = num.get_num();
        if state == FormulaBeforeState::Nega {
          result = result * -1;
        }
        formula.push_stack(FormulaType::Number(result));
        return self.formula_continue(num, formula, FormulaBeforeState::None);
      }
      Syntax::Str(strs) => {
        formula.push_stack(FormulaType::Strings(strs.get_str().into()));
        return self.formula_continue(strs, formula, FormulaBeforeState::None);
      }

      Syntax::Var(vars) => {
        let (serched_var, types) = self.serch_var(vars.get_name());
        let serched_var_inner = serched_var.ok_or(result::Error::InterpreterError(format!(
          "{} is not init",
          vars.get_name()
        )))?;

        if types?.is_none() {
          match &serched_var_inner {
            Syntax::Var(vars_2) => {
              return self.formula_object(vars, vars_2, formula);
            }
            _ => {
              return Err(result::Error::InterpreterError(format!(
                "{} is not found",
                vars.get_name()
              )));
            }
          }
        }

        self.formula_push(formula, &serched_var_inner, FormulaBeforeState::None)?;
        return self.formula_continue(vars, formula, FormulaBeforeState::None);
      }

      Syntax::Call(call) => {
        let inner = self
          .serch_fun(call.get_name())
          .ok_or(result::Error::InterpreterError(format!(
            "{} function not found",
            call.get_name()
          )))?;

        let function_return =
          self
            .function_run(&inner, call, None)?
            .ok_or(result::Error::InterpreterError(format!(
              "{} not a return value",
              call.get_name()
            )))?;

        self.formula_push(formula, &function_return, FormulaBeforeState::None)?;
        return self.formula_continue(call, formula, FormulaBeforeState::None);
      }

      _ => Err(result::Error::InterpreterError(
        "variable error cannot be assigned".to_string(),
      )),
    }
  }

  fn formula_continue<T: Node>(
    &mut self,
    node: &T,
    formula: &mut Formula,
    state: FormulaBeforeState,
  ) -> Result<(), result::Error> {
    match node.get_node_index(0) {
      Some(ast) => self.formula_push(formula, ast, state),
      None => {
        return Ok(());
      }
    }
  }

  fn formula_object<T: Node>(
    &mut self,
    node: &T,
    inner: &ast::VariableAST,
    formula: &mut Formula,
  ) -> Result<(), result::Error> {
    let var = node
      .get_node_index(0)
      .ok_or(result::Error::InterpreterError(format!(
        "the import value error"
      )))?;

    let bin: &ast::BinaryAST;
    match &var {
      &Syntax::Bin(b) => {
        bin = b;
      }

      _ => {
        return Err(result::Error::InterpreterError(format!(
          "the import value error"
        )))
      }
    }

    if bin.get_token() != TOKEN._dot {
      return Err(result::Error::InterpreterError(format!(
        "the import value error"
      )));
    }

    let bin_innner_node = bin
      .get_node_index(0)
      .ok_or(result::Error::InterpreterError(format!(
        "no member specified to access"
      )))?;

    match bin_innner_node {
      Syntax::Var(var) => {
        let serched_var =
          inner
            .serch_variable(var.get_name())
            .ok_or(result::Error::InterpreterError(format!(
              "{} is notfound",
              var.get_name()
            )))?;

        match serched_var {
          Syntax::Var(vars) => {
            return Err(result::Error::InterpreterError(format!(
              "{} can't access this",
              vars.get_name()
            )))
          }

          _ => {
            self.formula_push(formula, &serched_var, FormulaBeforeState::None)?;
            return self.formula_continue(var, formula, FormulaBeforeState::None);
          }
        }
      }

      Syntax::Call(call) => {
        let serched_function =
          inner
            .serch_functions(call.get_name())
            .ok_or(result::Error::InterpreterError(format!(
              "{} is notfound function",
              call.get_name()
            )))?;

        let function_return = self
          .function_run(serched_function, call, Some(inner))?
          .ok_or(result::Error::InterpreterError(format!(
            "{} is notfound return value",
            call.get_name(),
          )))?;

        self.formula_push(formula, &function_return, FormulaBeforeState::None)?;
        return self.formula_continue(call, formula, FormulaBeforeState::None);
      }

      _ => {
        return Err(result::Error::InterpreterError(format!(
          "no member specified to access"
        )))
      }
    }
  }
}
