use crate::vim::connector::Connector;
use crate::vim::vim::MethodHandler;
use rmpv::Value;
use std::rc::Rc;

pub struct CancelHandler {
  pub connector: Rc<Connector>,
}

impl MethodHandler for CancelHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value> {
    if args.len() >= 1 {
      if let Some(s) = args[0].as_str() {
        self.connector.cancel(s.to_string());

        Ok(Value::from("canceled"))
      } else {
        Err(Value::from("expect str"))
      }
    } else {
      Err(Value::from("expect context_id parameter"))
    }
  }
}
