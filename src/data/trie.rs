use crate::output::tree::{TreeDepth, TreeParam, TreeStream};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

type NodePtr<KeyType, T> = Rc<RefCell<_Node<KeyType, T>>>;

#[derive(Debug)]
pub struct _Node<KeyType: Hash + Eq + Clone, T> {
  children: HashMap<KeyType, NodePtr<KeyType, T>>,
  values: Vec<T>,
}

impl<KeyType: Hash + Eq + Clone, T> _Node<KeyType, T> {
  pub fn push(&mut self, value: T) {
    self.values.push(value)
  }

  pub fn child(&mut self, key: &KeyType) -> NodePtr<KeyType, T> {
    match self.children.get_mut(key) {
      Some(c) => c.clone(),
      None => {
        let c = Rc::new(RefCell::new(_Node {
          children: HashMap::new(),
          values: Vec::new(),
        }));
        self.children.insert(key.clone(), c.clone());
        c
      }
    }
  }
}

impl<KeyType: Hash + Eq + Clone + fmt::Display, T: fmt::Display> _Node<KeyType, T> {
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

#[derive(Debug)]
pub struct Trie<K: Hash + Eq + Clone, T> {
  root: NodePtr<K, T>,
}

impl<K: Hash + Eq + Clone, T> Trie<K, T> {
  pub fn create() -> Trie<K, T> {
    Trie {
      root: Rc::new(RefCell::new(_Node {
        children: HashMap::new(),
        values: Vec::new(),
      })),
    }
  }

  pub fn bfs(&self, root: NodePtr<K, T>) -> Vec<NodePtr<K, T>> {
    let mut res: Vec<NodePtr<K, T>> = Vec::new();
    let mut queue: VecDeque<NodePtr<K, T>> = VecDeque::new();
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

impl<'a, K: Hash + Eq + Clone + 'a, T> Trie<K, T> {
  pub fn insert<I>(&mut self, pattern: I, value: T)
  where
    I: Iterator<Item = &'a K>,
  {
    let mut cur = self.root.clone();
    for ch in pattern {
      let tmp = cur.borrow_mut().child(&ch);
      cur = tmp;
    }
    cur.borrow_mut().push(value);
  }
}
