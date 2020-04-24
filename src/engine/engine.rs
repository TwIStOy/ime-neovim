use crate::engine::candidate::Candidate;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

static mut _CONTEXT_IDX: u32 = 0;

#[derive(Clone)]
pub struct ContextId {
  id: u32,
}

impl ContextId {
  pub fn new() -> ContextId {
    unsafe {
      _CONTEXT_IDX += 1;
      ContextId { id: _CONTEXT_IDX }
    }
  }

  pub fn id(&self) -> u32 {
    self.id
  }
}

pub enum BackspaceResult {
  Candidates(Vec<Candidate>),
  Cancel,
}

pub trait InputContext: Send {
  fn feed(&mut self, ch: char) -> Vec<Candidate>;

  fn backspace(&mut self) -> BackspaceResult;

  fn id(&self) -> ContextId;
}

pub trait IMEngine: Send {
  fn start_context(&self) -> Rc<RefCell<dyn InputContext>>;

  fn start_context_async(&self) -> Arc<Mutex<dyn InputContext>>;
}
