use crate::data::trie::Trie;
use crate::engine::base::IFilter;
use crate::engine::candidate::Candidate;
use crate::engine::codetable::input_context::{CodeTableContext, ResultText};
use crate::engine::engine::{ContextId, IMEngine, InputContext};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

pub struct CodeTable {
  table: Arc<Mutex<Trie<char, ResultText>>>,
}

impl IMEngine for CodeTable {
  fn start_context(&mut self) -> Box<dyn InputContext> {
    // todo
    Box::new(CodeTableContext::new(self.table.lock().unwrap().root()))
  }
}

impl CodeTable {
  pub fn table_file(filename: &String) -> CodeTable {
    let mut code_table = CodeTable {
      table: Arc::new(Mutex::new(Trie::new())),
    };

    let mut filepath = dirs::home_dir().unwrap();
    filepath.push(".local");
    filepath.push("share");
    filepath.push("ime-neovim");
    filepath.push("codetable");
    filepath.push(filename);

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

        code_table.table.lock().unwrap().insert(
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
