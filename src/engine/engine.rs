use crate::engine::candidate::Candidate;

static mut ContextIdx: u32 = 0;

pub struct ContextId {
  id: u32,
}

impl ContextId {
  pub unsafe fn new() -> ContextId {
    ContextIdx += 1;
    ContextId { id: ContextIdx }
  }

  pub fn id(&self) -> u32 {
    self.id
  }
}

pub trait InputContext {
  fn feed(&mut self, ch: char) -> Vec<Candidate>;

  fn backspace(&mut self);
}

pub trait IMEngine {
  fn start_context(&mut self) -> dyn InputContext;

  fn cancel(&mut self);
}
