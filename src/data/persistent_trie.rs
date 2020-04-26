use log::info;
use std::collections::{HashMap, LinkedList};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct PersistentNode<K: Hash + Eq + Clone, V> {
  pub father: Option<Weak<Self>>,
  pub children: HashMap<K, Arc<Self>>,
  pub values: Vec<Arc<V>>,
}

impl<K: Hash + Eq + Clone, V> Clone for PersistentNode<K, V> {
  fn clone(&self) -> Self {
    Self {
      father: self.father.clone(),
      children: self.children.clone(),
      values: self.values.clone(),
    }
  }
}

impl<K: Hash + Eq + Clone, V> PersistentNode<K, V> {
  pub fn new() -> Self {
    PersistentNode {
      father: None,
      children: HashMap::new(),
      values: Vec::new(),
    }
  }

  pub fn child(&self, key: &K) -> Option<Arc<Self>> {
    Some(self.children.get(key)?.clone())
  }

  pub fn push(&self, v: V) -> Arc<Self> {
    let mut res = self.clone();
    res.values.push(Arc::new(v));
    Arc::new(res)
  }

  pub fn child_or_default(&self, key: K) -> Arc<Self> {
    if let Some(child) = self.children.get(&key) {
      child.clone()
    } else {
      let mut this = self.clone();
      this.children.insert(key, Arc::new(Self::new()));
      Arc::new(this)
    }
  }

  // bfs
  pub fn flatten(self: &Arc<Self>) -> Vec<Arc<Self>> {
    let mut res: Vec<Arc<Self>> = Vec::new();
    let mut queue: LinkedList<Arc<Self>> = LinkedList::new();
    queue.push_back(self.clone());

    while !queue.is_empty() {
      match queue.pop_front() {
        Some(cur) => {
          for child in cur.children.values() {
            queue.push_front(child.clone());
          }

          res.push(cur.clone());
        }
        None => {}
      }
    }

    res
  }
}

impl<K: Hash + Eq + Clone + Debug, V: Debug> PersistentNode<K, V> {
  pub fn maintain(self: Arc<Self>, father: Option<Weak<Self>>) {
    unsafe {
      let mut tmp = self.clone();
      (*Arc::get_mut_unchecked(&mut tmp)).father = father;
    }

    for (_, child) in &self.children {
      child.clone().maintain(Some(Arc::downgrade(&self)));
    }
  }
}

impl<'a, K: Hash + Eq + Clone + 'a, V> PersistentNode<K, V> {
  pub fn update<I>(&self, pattern: &mut I, value: V) -> Arc<Self>
  where
    I: Iterator<Item = &'a K>,
  {
    if let Some(ch) = pattern.next() {
      let mut this = self.clone();

      match this.children.get_mut(ch) {
        Some(child) => {
          *child = child.update(pattern, value);
        }
        None => {
          this
            .children
            .insert(ch.clone(), Self::new().update(pattern, value));
        }
      }

      Arc::new(this)
    } else {
      self.push(value)
    }
  }
}

#[derive(Debug)]
pub struct PersistentTrie<K: Hash + Eq + Clone, V> {
  root: Arc<PersistentNode<K, V>>,
}

impl<K: Hash + Eq + Clone, V> PersistentTrie<K, V> {
  pub fn new() -> Self {
    PersistentTrie {
      root: Arc::new(PersistentNode::<K, V>::new()),
    }
  }

  pub fn root(&self) -> Arc<PersistentNode<K, V>> {
    self.root.clone()
  }
}

impl<K: Hash + Eq + Clone + Debug, V: Debug> PersistentTrie<K, V> {
  pub fn maintain(&mut self) {
    // Arc::get_mut(&mut self.root).unwrap().maintain(None);
    self.root().maintain(None);
  }
}

impl<'a, K: Hash + Eq + Clone + 'a, V> PersistentTrie<K, V> {
  pub fn insert<I>(&self, mut pattern: I, value: V) -> Self
  where
    I: Iterator<Item = &'a K>,
  {
    PersistentTrie {
      root: self.root.update(&mut pattern, value),
    }
  }
}

// unsafe impl<K: Hash + Eq + Send, V: Send> Send for TrieNode<K, V> {}
// unsafe impl<K: Hash + Eq + Sync, V: Sync> Sync for TrieNode<K, V> {}
