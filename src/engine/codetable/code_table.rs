use crate::data::PersistentTrie;
use crate::engine::codetable::input_context::{CodeTableContext, ResultText};
use crate::engine::engine::{IMEngine, InputContext};
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use crate::path::LocalDataPath;

pub struct CodeTable {
  table: PersistentTrie<char, ResultText>,
}

impl IMEngine for CodeTable {
  fn start_context(&self) -> Rc<RefCell<dyn InputContext>> {
    // todo
    Rc::new(RefCell::new(CodeTableContext::new(self.table.root())))
  }
}

impl CodeTable {
  pub fn table_file(filename: &String) -> CodeTable {
    let mut code_table = CodeTable { table: PersistentTrie::new() };
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

        code_table.table.insert(
          v[1].chars().collect::<Vec<char>>().iter(),
          ResultText {
            text: v[0].to_string(),
            priority: priority,
          },
        );
      }
    }

    code_table
  }
}
