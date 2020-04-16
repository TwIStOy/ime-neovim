use crate::engine::candidate::Candidate;

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

  fn backspace(&mut self);

  fn id(&self) -> ContextId;
}

pub trait IMEngine {
  fn start_context(&mut self) -> Box<dyn InputContext>;
}
