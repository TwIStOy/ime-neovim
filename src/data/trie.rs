use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
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
