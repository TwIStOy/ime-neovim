use crate::vim::connector::Connector;
use crate::vim::vim::MethodHandler;
use rmpv::Value;
use std::rc::Rc;

pub struct StartContextHandler {
  pub connector: Rc<Connector>,
}

impl MethodHandler for StartContextHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value> {
    self.connector.start_context(args);

    Ok(Value::from("ok"))
  }
}
