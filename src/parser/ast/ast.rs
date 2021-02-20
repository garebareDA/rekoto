#[derive(Debug, Clone)]
//変数の型
pub enum Types {}

#[derive(Debug, Clone)]
//変数の型以外
pub enum Syntax {
  Var(VariableAST),
  Call(CallAST),
  Num(NumberAST),
  Str(StringAST),
  Bin(BinaryAST),
  Scope(ScopeAST),
  Ifs(Box<IfsAST>),
  Else(Box<ElseAST>),
  Elif(Box<ElifAST>),
}

#[derive(Debug, Clone)]
pub struct RootAST {
  node: Vec<Syntax>,
}

impl RootAST {
  pub fn new() -> Self {
    Self { node: Vec::new() }
  }

  pub fn get_node(&self) -> &Vec<Syntax> {
    &self.node
  }

  pub fn push_node(&mut self, node: Syntax) {
    self.node.push(node);
  }
}

#[derive(Debug, Clone)]
pub struct CallAST {
  name: String,
  argment: Vec<Syntax>,
  node: Vec<Syntax>,
}

impl CallAST {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      argment: Vec::new(),
      node: Vec::new(),
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_argment(&self) -> &Vec<Syntax> {
    &self.argment
  }

  pub fn get_node_index(&self, index: usize) -> &Syntax {
    &self.node[index]
  }

  pub fn push_argment(&mut self, argment: &Syntax) {
    self.argment.push(argment.clone());
  }

  pub fn push_node(&mut self, node: &Syntax) {
    self.node.push(node.clone());
  }
}

#[derive(Debug, Clone)]
pub struct VariableAST {
  name: String,
  mutable: bool,
  defined: bool,
  node: Vec<Syntax>,
}

impl VariableAST {
  pub fn new(name: &str, is_mutable: bool, is_def: bool) -> Self {
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

  pub fn get_node(&self) -> &Vec<Syntax> {
    &self.node
  }

  pub fn get_node_index(&self, index: usize) -> &Syntax {
    &self.node[index]
  }

  pub fn set_is_mutable(&mut self, is_mutable: bool) {
    self.mutable = is_mutable;
  }

  pub fn set_is_def(&mut self, is_def: bool) {
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

  pub fn get_node(&self) -> &Vec<Syntax> {
    &self.node
  }

  pub fn get_node_index(&self, index: usize) -> &Syntax {
    &self.node[index]
  }

  pub fn get_num(&self) -> i64 {
    self.num
  }
}

#[derive(Debug, Clone)]
pub struct StringAST {
  strs: String,
  node: Vec<Syntax>,
}

impl StringAST {
  pub fn new(str: &str) -> Self {
    Self {
      strs: str.to_string(),
      node: Vec::new(),
    }
  }

  pub fn get_str(&self) -> &str {
    &self.strs
  }

  pub fn push_node(&mut self, node: &Syntax) {
    self.node.push(node.clone());
  }
}

#[derive(Debug, Clone)]
pub struct BinaryAST {
  bin: String,
  node: Vec<Syntax>,
}

impl BinaryAST {
  pub fn new(bin: &str) -> Self {
    Self {
      bin:bin.to_string(),
      node: Vec::new(),
    }
  }

  pub fn len(&self) -> usize {
    self.node.len()
  }

  pub fn get_node(&self) -> &Vec<Syntax> {
    &self.node
  }

  pub fn get_node_index(&self, index: usize) -> &Syntax {
    &self.node[index]
  }

  pub fn push_node(&mut self, node: &Syntax) {
    self.node.push(node.clone());
  }

  pub fn get_bin(&self) -> &str {
    &self.bin
  }
}

#[derive(Debug, Clone)]
pub struct ScopeAST {
  scope:Vec<Syntax>,
}

impl ScopeAST {
  pub fn new() -> Self {
    Self{
      scope: Vec::new(),
    }
  }

  pub fn push_scope(&mut self, node: &Syntax) {
    self.scope.push(node.clone());
  }

  pub fn set_scope(&mut self, node: &Vec<Syntax>) {
    self.scope = node.to_vec();
  }

  pub fn get_scope(&self) -> &Vec<Syntax> {
    &self.scope
  }
}

#[derive(Debug, Clone)]
pub struct IfsAST {
  judge: Syntax,
  scope: Vec<Syntax>,
}

impl IfsAST {
  pub fn new(judge: Syntax) -> Self {
    Self {
      judge,
      scope: Vec::new(),
    }
  }

  pub fn push_scope(&mut self, node: &Syntax) {
    self.scope.push(node.clone());
  }

  pub fn get_scope(&self) -> &Vec<Syntax> {
    &self.scope
  }

  pub fn get_judge(&self) -> &Syntax {
    &self.judge
  }
}

#[derive(Debug, Clone)]
pub struct ElseAST {
  scope: Vec<Syntax>,
}

impl ElseAST {
  pub fn new() -> Self {
    Self {
      scope: Vec::new(),
    }
  }

  pub fn push_scope(&mut self, node: &Syntax) {
    self.scope.push(node.clone());
  }

  pub fn set_scope(&mut self, node: &Vec<Syntax>) {
    self.scope = node.to_vec();
  }

  pub fn get_scope(&self) -> &Vec<Syntax> {
    &self.scope
  }
}

#[derive(Debug, Clone)]
pub struct ElifAST {
  judge: Syntax,
  scope: Vec<Syntax>,
}

impl ElifAST {
  pub fn new(judge: Syntax) -> Self {
    Self {
      judge,
      scope: Vec::new(),
    }
  }

  pub fn push_scope(&mut self, node: &Syntax) {
    self.scope.push(node.clone());
  }

  pub fn set_scope(&mut self, node: &Vec<Syntax>) {
    self.scope = node.to_vec();
  }

  pub fn get_scope(&self) -> &Vec<Syntax> {
    &self.scope
  }

  pub fn get_judge(&self) -> &Syntax {
    &self.judge
  }
}