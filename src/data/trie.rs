use crate::output::tree::{TreeDepth, TreeParam, TreeStream};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct _Node<T> {
  children: HashMap<char, Rc<RefCell<_Node<T>>>>,
  values: Vec<T>,
}

impl<T> _Node<T> {
  pub fn push(&mut self, value: T) {
    self.values.push(value)
  }

  pub fn child(&mut self, key: char) -> Rc<RefCell<_Node<T>>> {
    self
      .children
      .entry(key)
      .or_insert(Rc::new(RefCell::new(_Node {
        children: HashMap::new(),
        values: Vec::new(),
      })))
      .clone()
  }
}

impl<T> _Node<T>
where
  T: fmt::Display,
{
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
pub struct Trie<T> {
  root: Rc<RefCell<_Node<T>>>,
}

impl<T> Trie<T> {
  pub fn create() -> Trie<T> {
    Trie {
      root: Rc::new(RefCell::new(_Node {
        children: HashMap::new(),
        values: Vec::new(),
      })),
    }
  }

  pub fn insert(&mut self, pattern: String, value: T) {
    let mut cur = self.root.clone();
    for ch in pattern.chars() {
      let tmp = cur.borrow_mut().child(ch);
      cur = tmp;
    }
    cur.borrow_mut().push(value);
  }

  pub fn bfs(&self, root: Rc<RefCell<_Node<T>>>) -> Vec<Rc<RefCell<_Node<T>>>> {
    let mut res: Vec<Rc<RefCell<_Node<T>>>> = Vec::new();
    let mut queue: VecDeque<Rc<RefCell<_Node<T>>>> = VecDeque::new();
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

impl<T> Trie<T>
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
