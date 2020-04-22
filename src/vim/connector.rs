use super::vim::{MethodHandler, Vim};
use crate::engine::candidate::Candidate;
use crate::engine::codetable::code_table::CodeTable;
use crate::engine::engine::{IMEngine, InputContext};
use log::info;
use rmpv::Value;
use serde::Deserialize;
use serde_json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Connector {
  vim: RefCell<Vim>,

  codetable: Box<dyn IMEngine>,
  contexts: RefCell<HashMap<String, Rc<RefCell<dyn InputContext>>>>,
}

struct StartContextHandler {
  connector: Rc<Connector>,
}

struct InputCharHandler {
  connector: Rc<Connector>,
}

struct BackspaceHandler {
  connector: Rc<Connector>,
}

struct CancelHandler {
  connector: Rc<Connector>,
}

impl MethodHandler for StartContextHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value> {
    self.connector.start_context(args);

    Ok(Value::from("ok"))
  }
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

impl MethodHandler for BackspaceHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value> {
    if args.len() >= 1 {
      if let Some(s) = args[0].as_str() {
        match self.connector.backspace(s.to_string()) {
          Some((canceled, candidates)) => {
            if canceled {
              self.connector.cancel(s.to_string());

              Ok(Value::from("cancel"))
            } else {
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

impl Connector {
  pub fn new() -> Rc<Connector> {
    // let session = Session::new_parent().unwrap();
    // let nvim = Neovim::new(session);
    let codetable: Box<dyn IMEngine> = Box::new(CodeTable::table_file(&"小鹤音形.txt".to_string()));

    let res = Rc::new(Connector {
      vim: RefCell::new(Vim::new()),
      codetable: codetable,
      contexts: RefCell::new(HashMap::new()),
    });

    res.vim.borrow_mut().register(
      "start_context".to_string(),
      Box::new(StartContextHandler {
        connector: res.clone(),
      }),
    );
    res.vim.borrow_mut().register(
      "input_char".to_string(),
      Box::new(InputCharHandler {
        connector: res.clone(),
      }),
    );
    res.vim.borrow_mut().register(
      "backspace".to_string(),
      Box::new(BackspaceHandler {
        connector: res.clone(),
      }),
    );
    res.vim.borrow_mut().register(
      "cancel".to_string(),
      Box::new(CancelHandler {
        connector: res.clone(),
      }),
    );

    res
  }

  pub fn recv(&self) {
    self.vim.borrow_mut().run();
  }

  fn start_context(&self, values: Vec<Value>) {
    if values.len() != 1 {
      info!(
        "failed to resolve 'StartContext' event arguments number, expect 1, but got {}",
        values.len()
      );
    }

    if let Some(id) = values[0].as_str() {
      self
        .contexts
        .borrow_mut()
        .insert(id.to_string(), self.codetable.start_context());
      info!("allocate context success");
    } else {
      info!("expect argument string, but {} got", values[0]);
    }
  }

  fn input_char(&self, context_id: String, ch: char) -> Option<Vec<Candidate>> {
    Some(
      self
        .contexts
        .borrow_mut()
        .get_mut(&context_id)?
        .borrow_mut()
        .feed(ch),
    )
  }

  fn backspace(&self, context_id: String) -> Option<(bool, Vec<Candidate>)> {
    Some(
      self
        .contexts
        .borrow_mut()
        .get_mut(&context_id)?
        .borrow_mut()
        .backspace(),
    )
  }

  fn cancel(&self, context_id: String) {
    self.contexts.borrow_mut().remove(&context_id);
  }
}
