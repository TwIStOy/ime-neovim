use crate::data::PersistentTrie;
use crate::path::LocalDataPath;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct AssistResult(String);

pub struct AssistCode {
  trie: PersistentTrie<char, AssistResult>,
  keycodes: HashSet<char>,
}

impl AssistCode {
  pub fn new(filename: &str) -> AssistCode {
    let mut assist_code = AssistCode {
      trie: PersistentTrie::new(),
      keycodes: HashSet::new(),
    };
    let filepath = LocalDataPath::new().sub("assist_code").file(filename);

    let reader = BufReader::new(File::open(filepath).unwrap());

    for l in reader.lines() {
      if let Ok(line) = l {
        let v: Vec<&str> = line.trim().split('\t').collect();

        let result = AssistResult(v[0].to_string());

        for i in 1..v.len() {
          assist_code.trie = assist_code
            .trie
            .insert(v[i].chars().collect::<Vec<char>>().iter(), result.clone());
          for ch in v[i].chars().collect::<Vec<char>>() {
            assist_code.keycodes.insert(ch);
          }
        }
      }
    }

    assist_code.trie.maintain();

    assist_code
  }

  pub fn keycodes(&self) -> HashSet<char> {
    self.keycodes.clone()
  }
}

