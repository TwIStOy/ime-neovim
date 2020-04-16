use crate::data::trie::{Trie, TrieNodePtr};
use crate::engine::candidate::Candidate;
use crate::engine::engine::{ContextId, InputContext};
use std::rc::Rc;

type NodeType = TrieNodePtr<char, String>;
pub struct CodeTableContext {
  id: ContextId,
  current: NodeType,
  input_sequence: Vec<char>,
}

impl CodeTableContext {
  pub fn new(node: NodeType) -> CodeTableContext {
    CodeTableContext {
      id: ContextId::new(),
      current: node,
      input_sequence: Vec::new(),
    }
  }
}

impl InputContext for CodeTableContext {
  fn feed(&mut self, ch: char) -> Vec<Candidate> {
    self.input_sequence.push(ch);
    let mut res: Vec<Candidate> = Vec::new();

    // todo

    res
  }

  fn backspace(&mut self) {
    let mut father: Option<NodeType> = None;

    match self.current.borrow().father.upgrade() {
      Some(f) => {
        father = Some(f.clone());
      }
      None => {}
    }

    if let Some(father) = father {
      self.current = father;
    }
  }

  fn id(&self) -> ContextId {
    self.id.clone()
  }
}
