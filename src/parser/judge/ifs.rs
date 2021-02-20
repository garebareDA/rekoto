use super::super::parsers::Parsers;
use super::super::ast::ast::Syntax;

impl Parsers {
  pub(crate) fn ifs(&mut self) {
    //TODO:Scopeの実装が終わったら
    self.index_inc();
    match self.judge() {
      Some(judge) => match judge {
        Ok(obj) => match obj {
          Syntax::Str(strs) => {

          }

          Syntax::Num(num) => {

          }

          _ => {

          }
        },

        Err(e) => {}
      },

      None => {}
    }
  }
}
