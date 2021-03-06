pub struct Token {
  pub _let: i64,
  pub _const: i64,
  pub _variable: i64,
  pub _number: i64,
  pub _string: i64,
  pub _if: i64,
  pub _else: i64,
  pub _elif: i64,
  pub _for: i64,
  pub _fn: i64,
  pub _import: i64,
  pub _return: i64,
  pub _break: i64,
  pub _true: i64,
  pub _false: i64,
  pub _empty: i64,
  pub _nega: i64,
  pub _add: i64,
  pub _sub: i64,
  pub _mul: i64,
  pub _div: i64,
  pub _sur: i64,
  pub _pipe: i64,
  pub _amp: i64,
  pub _equal: i64,
  pub _less_equ: i64,
  pub _greater_equ: i64,
  pub _greater: i64,
  pub _less: i64,
  pub _or: i64,
  pub _and: i64,
  pub _equ: i64,
  pub _not_equ: i64,
  pub _paren_left: i64,
  pub _paren_right: i64,
  pub _braces_left: i64,
  pub _braces_right: i64,
  pub _comma: i64,
  pub _colon: i64,
  pub _dot: i64,
  pub _end: i64,
}

impl Token {
  pub const fn new() -> Self {
    Self {
      _let: -1,
      _const: -2,
      _variable: -3,
      _number: -4,
      _string: -5,
      _if: -6,
      _else: -7,
      _elif: -8,
      _for: -9,
      _fn: -10,
      _less_equ: -11,
      _greater_equ: -12,
      _or: -13,
      _and: -14,
      _equ: -15,
      _not_equ: -16,
      _import: -17,
      _return: -18,
      _break: -19,
      _true: -20,
      _false: -21,
      _empty: 0,
      _nega: 33,
      _pipe: 124,
      _amp: 38,
      _add: 43,
      _sub: 45,
      _mul: 42,
      _div: 47,
      _sur: 37,
      _equal: 61,
      _greater: 62,
      _less: 60,
      _paren_left: 40,
      _paren_right: 41,
      _braces_left: 123,
      _braces_right: 125,
      _colon: 58,
      _comma: 44,
      _dot: 46,
      _end: 59,
    }
  }
}
