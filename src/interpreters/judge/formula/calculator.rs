use super::formula::{Formula, FormulaType};
use crate::error::result;

enum CalcuType {
    Strings((String, String)),
    Nums((i64, i64)),
    Bools((bool, bool)),
}

impl Formula {
    pub(crate) fn mul(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let num = self.type_equal_all(left, right, "*")?;
        match num {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Number(num.0 * num.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    "* oprator missmathced type not a number".to_string(),
                ));
            }
        }
    }

    pub(crate) fn div(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let num = self.type_equal_all(left, right, "/")?;
        match num {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Number(num.0 / num.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    "/ oprator missmathced type not a number".to_string(),
                ));
            }
        }
    }

    pub(crate) fn sur(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let num = self.type_equal_all(left, right, "%")?;
        match num {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Number(num.0 % num.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    "% oprator missmathced type not a number".to_string(),
                ));
            }
        }
    }

    pub(crate) fn sub(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let num = self.type_equal_all(left, right, "-")?;
        match num {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Number(num.0 - num.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    "- oprator missmathced type not a number".to_string(),
                ));
            }
        }
    }

    pub(crate) fn add(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        match left {
            FormulaType::Number(num) => match right {
                FormulaType::Number(num2) => return Ok(FormulaType::Number(num + num2)),
                FormulaType::Strings(strs) => {
                    return Ok(FormulaType::Strings(format!("{}{}", num, strs)))
                }
                _ => {
                    return Err(result::Error::InterpreterError(
                        "+ error left side is bool".to_string(),
                    ))
                }
            },

            FormulaType::Strings(strs) => match right {
                FormulaType::Number(num) => {
                    return Ok(FormulaType::Strings(format!("{}{}", strs, num)))
                }
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

    pub(crate) fn greater_equ(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let num = self.type_equal_all(left, right, ">=")?;
        match num {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Bool(num.0 >= num.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    ">= oprator missmathced type not a number".to_string(),
                ));
            }
        }
    }

    pub(crate) fn less_equ(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let num = self.type_equal_all(left, right, "<=")?;
        match num {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Bool(num.0 <= num.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    "<= oprator missmathced type not a number".to_string(),
                ));
            }
        }
    }

    pub(crate) fn less(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let num = self.type_equal_all(left, right, "<")?;
        match num {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Bool(num.0 < num.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    "< oprator missmathced type not a number".to_string(),
                ));
            }
        }
    }

    pub(crate) fn grater(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let num = self.type_equal_all(left, right, ">")?;
        match num {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Bool(num.0 > num.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    "> oprator missmathced type not a number".to_string(),
                ));
            }
        }
    }

    pub(crate) fn equal(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let all = self.type_equal_all(left, right, "==")?;
        match all {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Bool(num.0 == num.1));
            }

            CalcuType::Strings(strs) => {
                return Ok(FormulaType::Bool(strs.0 == strs.1));
            }

            CalcuType::Bools(bools) => {
                return Ok(FormulaType::Bool(bools.0 == bools.1));
            }
        }
    }

    pub(crate) fn not_equal(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let all = self.type_equal_all(left, right, "!=")?;
        match all {
            CalcuType::Nums(num) => {
                return Ok(FormulaType::Bool(num.0 != num.1));
            }

            CalcuType::Strings(strs) => {
                return Ok(FormulaType::Bool(strs.0 != strs.1));
            }

            CalcuType::Bools(bools) => {
                return Ok(FormulaType::Bool(bools.0 != bools.1));
            }
        }
    }

    pub(crate) fn and(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let all = self.type_equal_all(left, right, "&&")?;
        match all {
            CalcuType::Bools(bools) => {
                return Ok(FormulaType::Bool(bools.0 && bools.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    "&& oprator missmathced type not a number".into(),
                ));
            }
        }
    }

    pub(crate) fn or(
        &self,
        left: FormulaType,
        right: FormulaType,
    ) -> Result<FormulaType, result::Error> {
        let all = self.type_equal_all(left, right, "||")?;
        match all {
            CalcuType::Bools(bools) => {
                return Ok(FormulaType::Bool(bools.0 || bools.1));
            }

            _ => {
                return Err(result::Error::InterpreterError(
                    "|| oprator missmathced type not a number".into(),
                ));
            }
        }
    }

    fn type_equal_all(
        &self,
        left: FormulaType,
        right: FormulaType,
        op: &str,
    ) -> Result<CalcuType, result::Error> {
        match left {
            FormulaType::Number(num) => match right {
                FormulaType::Number(num2) => {
                    return Ok(CalcuType::Nums((num, num2)));
                }
                _ => {
                    return Err(result::Error::InterpreterError(format!(
                        "caluculation {} operator error {} missmatched type",
                        op, num
                    )))
                }
            },

            FormulaType::Strings(strs) => match right {
                FormulaType::Strings(strs2) => {
                    return Ok(CalcuType::Strings((strs, strs2)));
                }

                _ => {
                    return Err(result::Error::InterpreterError(format!(
                        "caluculation {} operator error {} missmatched type",
                        op, strs
                    )))
                }
            },

            FormulaType::Bool(bools) => match right {
                FormulaType::Bool(bools2) => {
                    return Ok(CalcuType::Bools((bools, bools2)));
                }

                _ => {
                    return Err(result::Error::InterpreterError(format!(
                        "caluculation {} operator error {} missmatched type",
                        op, bools,
                    )))
                }
            },

            _ => {
                return Err(result::Error::InterpreterError(
                    "caluclation opetor error possible interprter error".into(),
                ))
            }
        }
    }
}
