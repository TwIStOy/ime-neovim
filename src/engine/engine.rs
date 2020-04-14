use crate::engine::candidate::Candidate;
use crate::data::trie::Trie;

pub struct IMEngine {
  codes: Trie<String>,
}

impl IMEngine {
  pub fn feed(ch: char) -> Vec<Candidate> {

    vec![]
  }

  pub fn add_code(&mut self, codes: String, text: String) {
    self.codes.insert(codes, text);
  }
}