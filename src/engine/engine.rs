use crate::engine::candidate::Candidate;
use std::rc::Rc;
use std::cell::RefCell;

static mut _ContextIdx: u32 = 0;

#[derive(Clone)]
pub struct ContextId {
  id: u32,
}

impl ContextId {
  pub fn new() -> ContextId {
    unsafe {
      _ContextIdx += 1;
      ContextId { id: _ContextIdx }
    }
  }

  pub fn id(&self) -> u32 {
    self.id
  }
}

pub trait InputContext {
  fn feed(&mut self, ch: char) -> Vec<Candidate>;

  fn backspace(&mut self) -> (bool, Vec<Candidate>);

  fn id(&self) -> ContextId;
}

pub trait IMEngine {
  fn start_context(&self) -> Rc<RefCell<dyn InputContext>>;
}
