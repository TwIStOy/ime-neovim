use crate::data::PersistentTrie;
use crate::engine::codetable::input_context::{CodeTableContext, ResultText};
use crate::engine::engine::{IMEngine, InputContext};
use crate::engine::Configuration;
use crate::path::LocalDataPath;
use async_std::sync::Mutex;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::sync::Arc;

pub struct CodeTable {
  table: PersistentTrie<char, ResultText>,
  keycodes: HashSet<char>,
  perfect_only: bool,
}

impl IMEngine for CodeTable {
  fn start_context(&self) -> Rc<RefCell<dyn InputContext>> {
    // todo
    Rc::new(RefCell::new(CodeTableContext::new(self.table.root())))
  }

  fn start_context_async(&self) -> Arc<Mutex<dyn InputContext>> {
    // todo
    Arc::new(Mutex::new(CodeTableContext::new(self.table.root())))
  }

  fn keycodes(&self) -> HashSet<char> {
    self.keycodes.clone()
  }
}

impl CodeTable {
  pub fn new(config: Configuration) -> Option<CodeTable> {
    if let Configuration::CodeTable {
      perfect_only,
      codetable_file,
    } = config
    {
      let mut res = CodeTable::table_file(&codetable_file);
      res.perfect_only = perfect_only;
      Some(res)
    } else {
      None
    }
  }

  pub fn table_file(filename: &str) -> CodeTable {
    let mut code_table = CodeTable {
      table: PersistentTrie::new(),
      keycodes: HashSet::new(),
      perfect_only: false,
    };
    let filepath = LocalDataPath::new().sub("codetable").file(filename);

    let reader = BufReader::new(File::open(filepath).unwrap());

    for l in reader.lines() {
      if let Ok(line) = l {
        let v: Vec<&str> = line.trim().split('\t').collect();

        let priority;
        if v.len() == 2 {
          priority = 100;
        // panic!("line should be splited into two parts, but got {}", v.len());
        } else {
          priority = v[2].parse::<u32>().unwrap();
        }

        code_table.table = code_table.table.insert(
          v[1].chars().collect::<Vec<char>>().iter(),
          ResultText {
            text: v[0].to_string(),
            priority: priority,
          },
        );
        for ch in v[1].chars().collect::<Vec<char>>() {
          code_table.keycodes.insert(ch);
        }
      }
    }

    code_table.table.maintain();

    code_table
  }
}
