use rmpv::Value;
use serde::{Deserialize, Serialize};
use serde_json;

pub struct PinyinInitials(String);
pub struct PinyinFinals(String);

#[derive(Serialize, Deserialize, Debug)]
pub enum Configuration {
  CodeTable { perfect_only: bool, codefile: String, },
  Pinyin,
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
