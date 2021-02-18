#[derive(Debug, Clone)]
//変数の型
pub enum Types {}

#[derive(Debug, Clone)]
//変数の型以外
pub enum Syntax {
  Var(VariableAST),
  Num(NumberAST),
  Str(StringAST),
  Bin(BinaryAST),
}

#[derive(Debug, Clone)]
pub struct RootAST {
  node: Vec<Syntax>,
}

impl RootAST {
  pub fn new() -> Self {
    Self { node: Vec::new() }
  }

  pub fn push_node(&mut self, node: Syntax) {
    self.node.push(node);
  }
}

#[derive(Debug, Clone)]
pub struct VariableAST {
  name: String,
  mutable: bool,
  defined:bool,
  node: Vec<Syntax>,
}

impl VariableAST {
  pub fn new(name: &str, is_mutable: bool, is_def:bool) -> Self {
    Self {
      name: name.to_string(),
      mutable: is_mutable,
      defined: is_def,
      node: Vec::new(),
    }
  }

  pub fn push_node(&mut self, node: &Syntax) {
    self.node.push(node.clone());
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_is_mutable(&self) -> bool {
    self.mutable
  }

  pub fn get_is_def(&self) -> bool {
    self.defined
  }

  pub fn set_is_mutable(&mut self, is_mutable:bool) {
    self.mutable = is_mutable;
  }

  pub fn set_is_def(&mut self, is_def:bool) {
    self.defined = is_def;
  }
}

#[derive(Debug, Clone)]
pub struct NumberAST {
  num: i64,
  node: Vec<Syntax>,
}

impl NumberAST {
  pub fn new(num: i64) -> Self {
    Self {
      num,
      node: Vec::new(),
    }
  }

  pub fn push_node(&mut self, node: &Syntax) {
    self.node.push(node.clone());
  }

  pub fn get_num(&self) -> i64 {
    self.num
  }
}

#[derive(Debug, Clone)]
pub struct StringAST {
  str:String,
  node: Vec<Syntax>
}

impl StringAST {
  pub fn new(str:&str) -> Self {
    Self{
      str: str.to_string(),
      node:Vec::new(),
    }
  }

  pub fn get_str(&self) -> &str {
    self.get_str()
  }

  pub fn push_node(&mut self, node: &Syntax) {
    self.node.push(node.clone());
  }
}

#[derive(Debug, Clone)]
pub struct BinaryAST {
  bin:char,
  node:Vec<Syntax>,
}

impl BinaryAST {
  pub fn new(bin:char) -> Self {
    Self{
      bin,
      node: Vec::new(),
    }
  }

  pub fn len(&self) -> usize {
    self.node.len()
  }

  pub fn push_node(&mut self, node: &Syntax) {
    self.node.push(node.clone());
  }

  pub fn get_bin(&self) -> char {
    self.bin
  }
}