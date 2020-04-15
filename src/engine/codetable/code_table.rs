use crate::data::trie::Trie;
use crate::engine::candidate::Candidate;
use crate::engine::engine::IMEngine;

struct CodeTable {
  table: Trie<char, String>,
}

impl CodeTable {}

// impl IMEngine for CodeTable {
//   fn feed(&mut self, ch: char) -> Vec<Candidate> {
//     // todo

//     vec![]
//   }

//   fn reset(&mut self) {
//     // todo
//   }
// }
