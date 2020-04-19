use crate::data::trie::{Trie, TrieNodePtr, TrieNode};
use crate::engine::candidate::Candidate;
use crate::engine::engine::{ContextId, InputContext};
use std::cmp;
use std::collections::{LinkedList, VecDeque};
use std::rc::Rc;

pub struct ResultText {
  pub text: String,
  pub priority: u32,
}

type NodeType = TrieNodePtr<char, ResultText>;
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

struct QueueItem {
  node: NodeType,
  depth: usize,
  codes: Vec<char>,
  priority: u32,
}

#[derive(Debug)]
struct FlattenItem {
  text: String,
  depth: usize,
  codes: Vec<char>,
  priority: u32,
}

impl cmp::PartialEq for FlattenItem {
  fn eq(&self, rhs: &Self) -> bool {
    self.text == rhs.text
      && self.depth == rhs.depth
      && self.codes == rhs.codes
      && self.priority == rhs.priority
  }
}

impl cmp::PartialOrd for FlattenItem {
  fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
    if self.depth != rhs.depth {
      self.depth.partial_cmp(&rhs.depth)
    } else {
      rhs.priority.partial_cmp(&self.priority)
    }
  }
}

impl cmp::Eq for FlattenItem {}

impl cmp::Ord for FlattenItem {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl CodeTableContext {
  fn generate_candidates(&mut self) -> Vec<FlattenItem> {
    let mut res: Vec<FlattenItem> = Vec::new();

    let mut queue: LinkedList<QueueItem> = LinkedList::new();
    queue.push_back(QueueItem {
      node: self.current.clone(),
      depth: 0,
      codes: Vec::new(),
      priority: 10,
    });

    while queue.len() > 0 {
      let mut item = queue.pop_front();
      if let Some(front) = item.as_mut() {
        let front_codes = front.codes.clone();

        for text in &front.node.borrow().values {
          res.push(FlattenItem {
            text: text.text.clone(),
            depth: front.depth,
            codes: front_codes.clone(),
            priority: text.priority,
          });
        }

        for (ch, child) in &front.node.borrow().children {
          let mut codes = front_codes.clone();
          codes.push(*ch);

          queue.push_back(QueueItem {
            node: child.clone(),
            depth: front.depth + 1,
            codes: codes,
            priority: 1,
          });
        }
      }
    }

    res
  }
}

impl InputContext for CodeTableContext {
  fn feed(&mut self, ch: char) -> Vec<Candidate> {
    self.input_sequence.push(ch);

    self.current = TrieNode::<char, ResultText>::child(&self.current, &ch);

    let mut candidates = self.generate_candidates();
    candidates.sort();

    candidates
      .iter()
      .map(|item| Candidate::new(item.text.clone(), item.codes.clone()))
      .collect()
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
