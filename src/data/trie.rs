use crate::output::tree::{TreeDepth, TreeParam, TreeStream};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::hash::Hash;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct TrieNode<K: Hash + Eq, V> {
  pub father: Weak<RefCell<Self>>,
  pub children: HashMap<K, Rc<RefCell<Self>>>,
  pub values: Vec<V>,
}
pub type TrieNodePtr<K: Hash + Eq, V> = Rc<RefCell<TrieNode<K, V>>>;

pub struct Trie<K: Hash + Eq, V> {
  root: Rc<RefCell<TrieNode<K, V>>>,
}

impl<K: Hash + Eq, V> TrieNode<K, V> {
  pub fn push(&mut self, value: V) {
    self.values.push(value)
  }
}

impl<K: Hash + Eq + Clone, V> TrieNode<K, V> {
  pub fn child(this: &Rc<RefCell<Self>>, key: &K) -> Rc<RefCell<Self>> {
    let mut this_ref = this.borrow_mut();
    match this_ref.children.get_mut(key) {
      Some(c) => c.clone(),
      None => {
        let c = Rc::new(RefCell::new(TrieNode {
          father: Rc::downgrade(this),
          children: HashMap::new(),
          values: Vec::new(),
        }));

        this_ref.children.insert(key.clone(), c.clone());
        c
      }
    }
  }
}

impl<K: Hash + Eq, V> Trie<K, V> {
  pub fn new() -> Self {
    Trie {
      root: Rc::new(RefCell::new(TrieNode {
        father: Weak::default(),
        children: HashMap::new(),
        values: Vec::new(),
      })),
    }
  }

  pub fn bfs(root: TrieNodePtr<K, V>) -> Vec<TrieNodePtr<K, V>> {
    let mut res: Vec<TrieNodePtr<K, V>> = Vec::new();
    let mut queue: VecDeque<TrieNodePtr<K, V>> = VecDeque::new();
    queue.push_front(root);

    while !queue.is_empty() {
      let mut front = queue.pop_back();

      match front.as_mut() {
        Some(cur) => {
          for child in cur.borrow().children.values() {
            let c = child.clone();
            queue.push_front(c);
          }

          res.push(cur.clone());
        }
        None => {}
      }
    }

    res
  }

  pub fn root(&self) -> TrieNodePtr<K, V> {
    self.root.clone()
  }
}

impl<KeyType: Hash + Eq + Clone + fmt::Display, T: fmt::Display> TrieNode<KeyType, T> {
  pub fn print_tree(
    &self,
    codes: String,
    depth: TreeDepth,
    last: bool,
    tree: &mut TreeStream,
    res: &mut Vec<String>,
  ) {
    let mut line = String::new();
    let signs = tree.new_row(TreeParam::new(depth, last));

    for sign in signs {
      line.push_str(sign.ascii());
    }

    if codes.len() == 0 {
      line.push_str("- root -");
    } else {
      line.push_str(" ");
      line.push_str(&codes);
      let mut first = true;

      line.push_str(", values: [");
      for value in &self.values {
        if first {
          first = false;
        } else {
          line.push_str(", ");
        }
        line.push_str(&value.to_string());
      }
      line.push_str("]");
    }

    res.push(line);

    let mut id = 0;
    for (ch, child) in &self.children {
      id += 1;
      child.borrow().print_tree(
        codes.clone() + &ch.to_string(),
        depth.deeper(),
        id == self.children.len(),
        tree,
        res,
      );
    }
  }
}

impl<'a, K: Hash + Eq + Clone + 'a, V> Trie<K, V> {
  pub fn insert<I>(&mut self, pattern: I, value: V)
  where
    I: Iterator<Item = &'a K>,
  {
    let mut cur = self.root.clone();
    for ch in pattern {
      let tmp = TrieNode::child(&cur, ch);
      cur = tmp;
    }
    cur.borrow_mut().push(value);
  }
}

impl<K: Hash + Eq + Clone + fmt::Display, T> Trie<K, T>
where
  T: fmt::Display,
{
  pub fn print_tree(&self) -> Vec<String> {
    let mut tree = TreeStream::new();
    let mut res: Vec<String> = Vec::new();

    self
      .root
      .borrow()
      .print_tree(String::new(), TreeDepth::root(), true, &mut tree, &mut res);

    res
  }
}
