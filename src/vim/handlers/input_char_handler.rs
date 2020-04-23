use crate::vim::connector::Connector;
use crate::vim::vim::MethodHandler;
use rmpv::Value;
use std::rc::Rc;

pub struct InputCharHandler {
  pub connector: Rc<Connector>,
}

impl MethodHandler for InputCharHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value> {
    if args.len() < 2 {
      Err(Value::from("expect args but not found"))
    } else {
      if let Some(context_id) = args[0].as_str() {
        if let Some(ch) = args[1].as_str() {
          if ch.len() >= 1 {
            match self
              .connector
              .input_char(context_id.to_string(), ch.chars().next().unwrap())
            {
              Some(candidates) => Ok(Value::from(vec![
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
              ])),
              None => Ok(Value::from(Vec::<(Value, Value)>::new())),
            }
          } else {
            Err(Value::from("expect char"))
          }
        } else {
          Err(Value::from("second parameter should be char"))
        }
      } else {
        Err(Value::from("first parameter should be str"))
      }
    }
  }
}
