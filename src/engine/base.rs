use rmpv::Value;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

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
    fuzzy_syllables: HashMap<String, String>,
    // character database filename
    character_database: String,
    // word database filename
    word_database: Option<String>,
    // if enable dynamic word frequency
    dynamic_word_frequency: bool,
  },
}

impl Configuration {
  pub fn new(value: &Value) -> Result<Self, String> {
    match value.as_str() {
      Some(v) => {
        match serde_json::from_str::<Configuration>(v) {
          Ok(res) => Ok(res),
          Err(e) => {
            Err(e.to_string())
          }
        }
      },
      None => Err("expect json string".to_string()),
    }
  }
}
