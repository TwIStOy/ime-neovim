use crate::data::trie::{Trie, TrieNode, TrieNodePtr};
use crate::engine::candidate::Candidate;
use crate::engine::engine::{ContextId, InputContext};
use log::info;
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
  overflow_number: u32,
}

impl CodeTableContext {
  pub fn new(node: NodeType) -> CodeTableContext {
    CodeTableContext {
      id: ContextId::new(),
      current: node,
      input_sequence: Vec::new(),
      overflow_number: 0,
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
    if self.input_sequence.len() == 0 {
      return vec![];
    }

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

    if self.overflow_number > 0 {
      self.overflow_number += 1
    } else {
      self.current = match TrieNode::<char, ResultText>::const_child(&self.current, &ch) {
        Some(child) => child,
        None => {
          self.overflow_number += 1;

          self.current.clone()
        }
      };
    }

    info!(
      "feed {}, input_seq: {:?}, overflow: {}",
      ch, self.input_sequence, self.overflow_number
    );

    if self.overflow_number > 0 {
      vec![]
    } else {
      let mut candidates = self.generate_candidates();
      candidates.sort();

      candidates
        .iter()
        .map(|item| Candidate::new(item.text.clone(), item.codes.clone()))
        .collect()
    }
  }

  fn backspace(&mut self) -> (bool, Vec<Candidate>) {
    self.input_sequence.pop();
    
    if self.input_sequence.len() == 0 {
      return (false, vec![])
    }

    let mut this_round = false;
    if self.overflow_number > 0 {
      self.overflow_number -= 1;
      this_round = true;
    }

    info!(
      "backspace, input_seq: {:?}, overflow: {}",
      self.input_sequence, self.overflow_number
    );

    if self.overflow_number == 0 {
      if !this_round {
        let mut father: Option<NodeType> = None;

        match self.current.borrow().father.upgrade() {
          Some(f) => {
            father = Some(f.clone());
          }
          None => return (true, vec![]),
        }

        if let Some(father) = father {
          self.current = father;
        }
      }

      let mut candidates = self.generate_candidates();
      candidates.sort();

      (true, candidates
        .iter()
        .map(|item| Candidate::new(item.text.clone(), item.codes.clone()))
        .collect())
    } else {
      (true, vec![])
    }
  }

  fn id(&self) -> ContextId {
    self.id.clone()
  }
}
