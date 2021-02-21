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
  For(Box<ForsAST>),
  Fn(FunctionAST),
  Return(Box<ReturnAST>),
  Break,
}

pub trait Node {
  fn get_node(&self) -> &Vec<Syntax>;
  fn get_node_index(&self, index: usize) -> &Syntax;
  fn get_node_len(&self) -> usize;
  fn push_node(&mut self, node: Syntax);
}

#[derive(Debug, Clone)]
pub struct RootAST {
  node: Vec<Syntax>,
}

impl RootAST {
  pub fn new() -> Self {
    Self { node: Vec::new() }
  }
}

impl Node for RootAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.node
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
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
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
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

  pub fn push_argment(&mut self, argment: &Syntax) {
    self.argment.push(argment.clone());
  }
}

impl Node for CallAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.node
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.node.push(node);
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
  pub fn new(name: impl Into<String>, is_mutable: bool, is_def: bool) -> Self {
    Self {
      name: name.into(),
      mutable: is_mutable,
      defined: is_def,
      node: Vec::new(),
    }
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

  pub fn set_is_mutable(&mut self, is_mutable: bool) {
    self.mutable = is_mutable;
  }

  pub fn set_is_def(&mut self, is_def: bool) {
    self.defined = is_def;
  }
}

impl Node for VariableAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.node
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.node.push(node);
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

  pub fn get_num(&self) -> i64 {
    self.num
  }
}

impl Node for NumberAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.node
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.node.push(node);
  }
}

#[derive(Debug, Clone)]
pub struct StringAST {
  strs: String,
  node: Vec<Syntax>,
}

impl StringAST {
  pub fn new(str: impl Into<String>) -> Self {
    Self {
      strs: str.into(),
      node: Vec::new(),
    }
  }

  pub fn get_str(&self) -> &str {
    &self.strs
  }
}

impl Node for StringAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.node
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.node.push(node);
  }
}

#[derive(Debug, Clone)]
pub struct BinaryAST {
  bin: String,
  node: Vec<Syntax>,
}

impl BinaryAST {
  pub fn new(bin: impl Into<String>) -> Self {
    Self {
      bin:bin.into(),
      node: Vec::new(),
    }
  }

  pub fn get_bin(&self) -> &str {
    &self.bin
  }
}

impl Node for BinaryAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.node
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.node.push(node);
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
}

impl Node for ScopeAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.scope
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.scope.push(node);
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

  pub fn get_judge(&self) -> &Syntax {
    &self.judge
  }
}

impl Node for IfsAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.scope
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.scope.push(node);
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
}

impl Node for ElseAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.scope
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.scope.push(node);
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

  pub fn get_judge(&self) -> &Syntax {
    &self.judge
  }
}

impl Node for ElifAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.scope
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.scope.push(node);
  }
}

#[derive(Debug, Clone)]
pub struct ForsAST {
  init:Syntax,
  judge:Syntax,
  add:Syntax,
  scope:Vec<Syntax>,
}

impl ForsAST {
  pub fn new(init:Syntax, judge:Syntax, add:Syntax) -> Self {
    Self {
      init,
      judge,
      add,
      scope:Vec::new(),
    }
  }

  pub fn get_init(&self) -> &Syntax {
    &self.init
  }

  pub fn get_judge(&self) -> &Syntax {
    &self.judge
  }

  pub fn get_add(&self) -> &Syntax {
    &self.add
  }
}

impl Node for ForsAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.scope
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.scope.push(node);
  }
}

#[derive(Debug, Clone)]
pub struct FunctionAST {
  name: String,
  param: Vec<Syntax>,
  scope:Vec<Syntax>,
}

impl FunctionAST {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name:name.into(),
      param:Vec::new(),
      scope:Vec::new(),
    }
  }

  pub fn push_param(&mut self, node: &Syntax) {
    self.param.push(node.clone());
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_param(&self) -> &Vec<Syntax> {
    &self.param
  }
}

impl Node for FunctionAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.scope
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.scope.push(node);
  }
}

#[derive(Debug, Clone)]
pub struct ReturnAST {
  node:Vec<Syntax>,
}

impl ReturnAST {
 pub fn new() -> Self {
  Self {
    node:Vec::new(),
  }
 }
}

impl Node for ReturnAST{
  fn get_node(&self) -> &Vec<Syntax> {
    &self.node
  }

  fn get_node_index(&self, index:usize) -> &Syntax {
    &self.get_node()[index]
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.node.push(node);
  }
}