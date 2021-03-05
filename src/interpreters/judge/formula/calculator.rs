use super::formula::{Formula, FormulaType};
use crate::error::result;

impl Formula {
  pub fn mul(&self, left: FormulaType, right: FormulaType) -> Result<FormulaType, result::Error> {
    let num = self.type_equal_number(left, right,"*")?;
    return Ok(FormulaType::Number(num.0 * num.1));
  }

  pub fn div(&self, left: FormulaType, right: FormulaType) -> Result<FormulaType, result::Error> {
    let num = self.type_equal_number(left, right, "/")?;
    return Ok(FormulaType::Number(num.0 / num.1));
  }

  pub fn sur(&self, left: FormulaType, right: FormulaType) -> Result<FormulaType, result::Error> {
    let num = self.type_equal_number(left, right, "%")?;
    return Ok(FormulaType::Number(num.0 % num.1));
  }

  pub fn sub(&self, left: FormulaType, right: FormulaType) -> Result<FormulaType, result::Error> {
    let num = self.type_equal_number(left, right, "-")?;
    return Ok(FormulaType::Number(num.0 - num.1));
  }

  pub fn add(&self, left: FormulaType, right: FormulaType) -> Result<FormulaType, result::Error> {
    match left {
      FormulaType::Number(num) => match right {
        FormulaType::Number(num2) => return Ok(FormulaType::Number(num + num2)),
        FormulaType::Strings(strs) => return Ok(FormulaType::Strings(format!("{}{}", num, strs))),
        _ => {
          return Err(result::Error::InterpreterError(
            "+ error left side is bool".to_string(),
          ))
        }
      },

      FormulaType::Strings(strs) => match right {
        FormulaType::Number(num) => return Ok(FormulaType::Strings(format!("{}{}", strs, num))),
        FormulaType::Strings(strs2) => {
          return Ok(FormulaType::Strings(format!("{}{}", strs, strs2)))
        }
        _ => {
          return Err(result::Error::InterpreterError(
            "+ error right side is bool".to_string(),
          ))
        }
      },

      _ => {
        return Err(result::Error::InterpreterError(
          "+ error right side is bool".to_string(),
        ))
      }
    }
  }

  pub fn type_equal_number(
    &self,
    left: FormulaType,
    right: FormulaType,
    op: &str,
  ) -> Result<(i64, i64), result::Error> {
    match left {
      FormulaType::Number(num) => match right {
        FormulaType::Number(num2) => {
          return Ok((num, num2));
        }
        _ => {
          return Err(result::Error::InterpreterError(format!(
            "caluculation {} operator error {}",
            op, num
          )))
        }
      },
      _ => {
        return Err(result::Error::InterpreterError(format!(
          "caluculation {} operator error",
          op
        )))
      }
    }
  }
}
