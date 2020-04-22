use super::vim::{MethodHandler, Vim};
use crate::engine::codetable::code_table::CodeTable;
use crate::engine::engine::{IMEngine, InputContext};
use log::info;
use rmpv::Value;
use serde::Deserialize;
use serde_json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct Connector {
  vim: RefCell<Vim>,

  codetable: Box<dyn IMEngine>,
  contexts: HashMap<String, Box<dyn InputContext>>,
}

struct StartContextHandler {
  connector: Rc<RefCell<Connector>>,
}

impl MethodHandler for StartContextHandler {
  fn handle(&mut self, args: Vec<Value>) -> Result<Value, Value> {
    match self.connector.try_borrow_mut() {
      Ok(mut v) => {
        v.start_context(args);
      }
      Err(e) => {
        info!("failed");
      }
    }

    Ok(Value::from("ok"))
  }
}

impl Connector {
  pub fn new() -> Rc<RefCell<Connector>> {
    // let session = Session::new_parent().unwrap();
    // let nvim = Neovim::new(session);
    let codetable: Box<dyn IMEngine> = Box::new(CodeTable::table_file(&"小鹤音形.txt".to_string()));

    let res = Rc::new(RefCell::new(Connector {
      vim: RefCell::new(Vim::new()),
      codetable: codetable,
      contexts: HashMap::new(),
    }));

    res.borrow_mut().vim.borrow_mut().register(
      "start_context".to_string(),
      Box::new(StartContextHandler {
        connector: res.clone(),
      }),
    );

    res
  }

  pub fn recv(&self) {
    self.vim.borrow_mut().run();
  }

  fn start_context(&mut self, values: Vec<Value>) {
    if values.len() != 1 {
      info!(
        "failed to resolve 'StartContext' event arguments number, expect 1, but got {}",
        values.len()
      );
    }

    if let Some(id) = values[0].as_str() {
      self
        .contexts
        .insert(id.to_string(), self.codetable.start_context());
      info!("allocate context success");
    } else {
      info!("expect argument string, but {} got", values[0]);
    }
  }
}
