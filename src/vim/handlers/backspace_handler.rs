use crate::engine::engine::BackspaceResult;
use crate::vim::connector::Connector;
use crate::vim::vim::MethodHandler;
use rmpv::Value;
use std::rc::Rc;

pub struct BackspaceHandler {
  pub connector: Rc<Connector>,
}

impl MethodHandler for BackspaceHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value> {
    if args.len() >= 1 {
      if let Some(s) = args[0].as_str() {
        match self.connector.backspace(s.to_string()) {
          Some(result) => {
            match result {
              BackspaceResult::Candidates(candidates) => {
                Ok(Value::from(vec![
                  // todo(hawtian): split char sets
                  (Value::from("char_sets"), Value::from(Vec::<Value>::new())),
                  (
                    Value::from("candidates"),
                    Value::from(
                      candidates
                        .iter()
                        .map(|x| Value::from(x))
                        .collect::<Vec<Value>>(),
                    ),
                  ),
                ]))
              }
              BackspaceResult::Cancel => {
                self.connector.cancel(s.to_string());

                Ok(Value::from("cancel"))
              }
            }
          }
          None => Ok(Value::from(Vec::<Value>::new())),
        }
      } else {
        Err(Value::from("expect str"))
      }
    } else {
      Err(Value::from("expect context_id parameter"))
    }
  }
}
