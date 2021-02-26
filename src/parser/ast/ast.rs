#[derive(Debug, Clone, PartialEq)]
//変数の型
pub enum Types {
  Number,
  String,
  Bool,
}

#[derive(Debug, Clone)]
//変数の型以外
pub enum Syntax {
  Var(VariableAST),
  Call(CallAST),
  Num(NumberAST),
  Str(StringAST),
  Bool(BoolAST),
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
  fn get_node_index(&self, index: usize) -> Option<&Syntax>;
  fn get_node_len(&self) -> usize;
  fn push_node(&mut self, node: Syntax);
}

pub trait Type {
  fn get_type(&self) -> &Option<Types>;
  fn set_type(&mut self, types:Option<Types>);
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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

  pub fn get_argment_len(&self) -> usize {
    self.argment.len()
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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
  types: Option<Types>,
  node: Vec<Syntax>,
}

impl VariableAST {
  pub fn new(name: impl Into<String>, is_mutable: bool, is_def: bool) -> Self {
    Self {
      name: name.into(),
      mutable: is_mutable,
      defined: is_def,
      types: None,
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.node.push(node);
  }
}

impl Type for VariableAST {
   fn set_type(&mut self, types: Option<Types>) {
    self.types = types;
  }

  fn get_type(&self) -> &Option<Types> {
    &self.types
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.node.push(node);
  }
}

#[derive(Debug, Clone)]
pub struct BoolAST {
  boolean:bool,
  node:Vec<Syntax>,
}

impl BoolAST {
  pub fn new(boolean:bool) -> Self {
    Self {
      boolean: boolean,
      node:Vec::new(),
    }
  }

  pub fn get_bool(&self) -> bool {
    self.boolean
  }
}

impl Node for BoolAST{
  fn get_node(&self) ->&Vec<Syntax> {
    &self.node
  }

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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
  token:i64,
  node: Vec<Syntax>,
}

impl BinaryAST {
  pub fn new(bin: impl Into<String>, token:i64) -> Self {
    Self {
      bin:bin.into(),
      token,
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
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
  types:Option<Types>
}

impl FunctionAST {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name:name.into(),
      param:Vec::new(),
      scope:Vec::new(),
      types:None,
    }
  }

  pub fn push_param(&mut self, node: Syntax) {
    self.param.push(node);
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.scope.push(node);
  }
}

impl Type for FunctionAST {
  fn set_type(&mut self, types: Option<Types>) {
   self.types = types;
 }

 fn get_type(&self) -> &Option<Types> {
   &self.types
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

  fn get_node_index(&self, index:usize) -> Option<&Syntax> {
    self.get_node().get(index)
  }

  fn get_node_len(&self) -> usize {
    self.get_node().len()
  }

  fn push_node(&mut self, node:Syntax) {
    self.node.push(node);
  }
}