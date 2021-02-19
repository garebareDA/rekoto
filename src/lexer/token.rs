pub struct Token {
  pub _let: i64,
  pub _const:i64,
  pub _variable:i64,
  pub _number:i64,
  pub _string:i64,
  pub _add:i64,
  pub _sub:i64,
  pub _mul:i64,
  pub _div:i64,
  pub _equal:i64,
  pub _paren_left:i64,
  pub _paren_right:i64,
  pub _comma: i64,
  pub _end:i64,
}

impl Token {
  pub const fn new() -> Self {
    Self {
      _let: -1,
      _const:-2,
      _variable:-3,
      _number: -4,
      _string:-5,
      _add: 43,
      _sub: 45,
      _mul: 42,
      _div: 47,
      _equal: 61,
      _paren_left: 40,
      _paren_right:41,
      _comma:44,
      _end: 59,
    }
  }
}
