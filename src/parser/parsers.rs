extern crate lelex;
use super::ast;
use crate::error::result;
use crate::parser::ast::ast::Node;
use lelex::tokens::Tokens;

#[derive(PartialEq, Debug)]
pub enum ParseState {
    Main,
    If,
    For,
    Var,
    Function,
    Call,
    Scope,
    Struct,
    Member,
    New,
}

pub struct Parsers {
    tokens: Vec<lelex::tokens::Tokens>,
    index: i64,
    state: Vec<ParseState>,
}

impl Parsers {
    pub fn new(tokens: Vec<lelex::tokens::Tokens>) -> Self {
        Self {
            tokens,
            index: 0,
            state: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<ast::ast::RootAST, result::Error> {
        let mut root = ast::ast::RootAST::new();
        let len = self.tokens.len();
        self.push_state(ParseState::Main);

        loop {
            match self.judge() {
                Some(result) => root.push_node(result?),
                None => {}
            }

            self.index_inc();
            if len <= self.index as usize {
                break;
            }
        }

        return Ok(root);
    }

    pub(crate) fn push_state(&mut self, state: ParseState) {
        self.state.push(state);
    }

    pub(crate) fn pop_state(&mut self) {
        self.state.pop();
    }

    pub(crate) fn get_last_state(&self) -> &ParseState {
        &self.state[self.state.len() - 1]
    }

    pub(crate) fn get_index(&self) -> i64 {
        self.index
    }

    pub(crate) fn get_tokens_len(&self) -> usize {
        self.tokens.len()
    }

    pub(crate) fn index_inc(&mut self) {
        self.index += 1;
    }

    pub(crate) fn index_add(&mut self, index: usize) {
        self.index += index as i64;
    }

    pub(crate) fn get_tokens(&self, num: i64) -> Option<&Tokens> {
        return self.tokens.get(num as usize);
    }
}
