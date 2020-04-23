use crate::vim::connector::Connector;
use crate::vim::vim::MethodHandler;
use rmpv::Value;
use std::rc::Rc;
use crate::engine::Configuration;

pub struct InitializeHandler {
  pub connector: Rc<Connector>,
}

impl MethodHandler for InitializeHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value> {
    if args.len() < 1 {
      Err(Value::from("expect init arguments but not found"))
    } else {
      match self.connector.initialize(Configuration::new(&args[0])?) {
        Ok(_) => Ok(Value::from("ok")),
        Err(s) => Err(Value::from(s))
      }
    }
  }
}
