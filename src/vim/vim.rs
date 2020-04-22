use super::model;
use log::info;
use rmpv::Value;
use std::collections::HashMap;
use std::io;
use std::io::{BufReader, BufWriter, Read, Stdin, Stdout, Write};

pub trait MethodHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value>;
}

struct DefaultHandler {}
impl MethodHandler for DefaultHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value> {
    Err(Value::from("not impl"))
  }
}

pub struct Vim {
  reader: BufReader<Stdin>,
  writer: BufWriter<Stdout>,
  handler: HashMap<String, Box<dyn MethodHandler>>,
}

impl Vim {
  pub fn new() -> Vim {
    Vim {
      reader: BufReader::new(io::stdin()),
      writer: BufWriter::new(io::stdout()),
      handler: HashMap::new(),
    }
  }

  pub fn register(&mut self, method: String, handler: Box<dyn MethodHandler>) {
    self.handler.insert(method, handler);
  }

  pub fn run(&mut self) {
    loop {
      let msg = match model::decode(&mut self.reader) {
        Ok(msg) => msg,
        Err(e) => {
          info!("{}", e);
          panic!("{}", e);
        }
      };

      info!("msg: {:?}", msg);

      match self.emit(msg) {
        Some(response) => {
          info!("msg: {:?}", response);
          model::encode(&mut self.writer, response);
        }
        None => {}
      }
    }
  }

  fn emit(&mut self, msg: model::RpcMessage) -> Option<model::RpcMessage> {
    match msg {
      model::RpcMessage::Request { id, method, params } => {
        match self.handler.get_mut(&method) {
          Some(handler) => {
            info!("handler found");
            match handler.handle(params) {
              Ok(result) => {
                info!("res: {}", result);
                Some(model::RpcMessage::Response {
                  id,
                  result,
                  error: Value::Nil,
                })
              }
              Err(e) => {
                info!("err: {}", e);
                Some(model::RpcMessage::Response {
                  id,
                  result: Value::Nil,
                  error: e,
                })
              }
            }
          }
          None => {
            info!("no handler for {}", method);

            // response error
            Some(model::RpcMessage::Response {
              id: id,
              result: Value::Nil,
              error: Value::from("not impl"),
            })
          }
        }
      }
      model::RpcMessage::Response { id, error, result } => None,
      model::RpcMessage::Notification { method, params } => None,
    }
  }
}
