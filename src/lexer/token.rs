pub struct Token {
  pub _let: i64,
  pub _const:i64,
  pub _variable:i64,
  pub _number:i64,
  pub _equal: i64,
}

impl Token {
  pub const fn new() -> Self {
    Self {
      _let: -1,
      _const:-2,
      _variable:-3,
      _number: -4,
      _equal: 61,
    }
  }
}
