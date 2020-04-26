use super::handlers::*;
use super::vim::Vim;
use crate::engine::candidate::Candidate;
use crate::engine::codetable::code_table::CodeTable;
use crate::engine::engine::{BackspaceResult, IMEngine, InputContext};
use crate::engine::Configuration;
use log::info;
use rmpv::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Connector {
  vim: RefCell<Vim>,

  engine: RefCell<Option<Box<dyn IMEngine>>>,
  contexts: RefCell<HashMap<String, Rc<RefCell<dyn InputContext>>>>,
}

impl Connector {
  pub fn new() -> Rc<Connector> {
    // let session = Session::new_parent().unwrap();
    // let nvim = Neovim::new(session);

    let res = Rc::new(Connector {
      vim: RefCell::new(Vim::new()),
      engine: RefCell::new(None),
      contexts: RefCell::new(HashMap::new()),
    });

    res.vim.borrow_mut().register(
      "initialize".to_string(),
      Box::new(InitializeHandler {
        connector: res.clone(),
      }),
    );
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

  pub fn initialize(&self, config: Configuration) -> Result<(), &str> {
    let engine = self.engine.borrow_mut();

    if engine.is_some() {
      Err("ime-neovim has been initialized")
    } else {
      match config {
        Configuration::CodeTable {
          perfect_only,
          codefile,
        } => {
          let codetable: Box<dyn IMEngine> = Box::new(CodeTable::table_file(&codefile));
          *self.engine.borrow_mut() = Some(codetable);

          Ok(())
        }
        Configuration::Pinyin => Err("not impl"),
      }
    }
  }

  pub fn start_context(&self, values: Vec<Value>) {
    if values.len() != 1 {
      info!(
        "failed to resolve 'StartContext' event arguments number, expect 1, but got {}",
        values.len()
      );
    }

    if let Some(id) = values[0].as_str() {
      if let Some(engine) = &*self.engine.borrow() {
        self
          .contexts
          .borrow_mut()
          .insert(id.to_string(), engine.start_context());
        info!("allocate context success");
      }
    } else {
      info!("expect argument string, but {} got", values[0]);
    }
  }

  pub fn input_char(&self, context_id: String, ch: char) -> Option<Vec<Candidate>> {
    Some(
      self
        .contexts
        .borrow_mut()
        .get_mut(&context_id)?
        .borrow_mut()
        .feed(ch)
        .0,
    )
  }

  pub fn backspace(&self, context_id: String) -> Option<BackspaceResult> {
    Some(
      self
        .contexts
        .borrow_mut()
        .get_mut(&context_id)?
        .borrow_mut()
        .backspace(),
    )
  }

  pub fn cancel(&self, context_id: String) {
    self.contexts.borrow_mut().remove(&context_id);
  }
}
