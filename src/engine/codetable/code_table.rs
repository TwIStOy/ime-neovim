use crate::data::trie::Trie;
use crate::engine::base::IFilter;
use crate::engine::candidate::Candidate;
use crate::engine::codetable::input_context::CodeTableContext;
use crate::engine::engine::{ContextId, IMEngine, InputContext};

struct CodeTable {
  table: Trie<char, String>,
}

impl IMEngine for CodeTable {
  fn start_context(&mut self) -> Box<dyn InputContext> {
    // todo
    Box::new(CodeTableContext::new(self.table.root()))
  }
}

// impl IMEngine for CodeTable {
//   fn feed(&mut self, ch: char) -> Vec<Candidate> {
//     // todo

//     vec![]
//   }

//   fn reset(&mut self) {
//     // todo
//   }
// }
