use crate::path::LocalConfigPath;
use rmpv::Value;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub enum Configuration {
  CodeTable {
    // enable to show only perfect matched candidates
    perfect_only: bool,
    // codetable filename
    codetable_file: String,
  },
  Pinyin {
    // KeyMap filename
    scheme_file: String,
    // AssistCodes filename
    assist_file: Option<String>,
    // fuzzy syllables settings
    fuzzy_syllables: Vec<(String, String)>,
    // character database filename
    character_database: String,
    // word database filename
    word_database: Option<String>,
    // if enable dynamic word frequency
    dynamic_word_frequency: bool,
  },
}

impl Configuration {
  pub fn new(filename: &Value) -> Result<Self, String> {
    let filepath = LocalConfigPath::new().file("config.json");
    let reader = BufReader::new(File::open(filepath).unwrap());

    let res: Self =
      serde_json::from_reader(reader).map_err(|_| "failed to read config.json".to_string())?;

    Ok(res)
  }
}
