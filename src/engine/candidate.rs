use rmpv::Value;
use rmpv::ValueRef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MatchType {
  PerfectMatch,
  PrefixMatch,
  FuzzyMatch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
  pub remain_codes: Vec<char>,
  pub text: String,
  pub match_type: MatchType,
}

impl Candidate {
  pub fn prefect(text: String) -> Candidate {
    Candidate {
      remain_codes: Vec::new(),
      text: text,
      match_type: MatchType::PerfectMatch,
    }
  }

  pub fn prefix(text: String, remain: Vec<char>) -> Candidate {
    Candidate {
      remain_codes: remain,
      text: text,
      match_type: MatchType::PrefixMatch,
    }
  }

  pub fn new(text: String, remain_codes: Vec<char>) -> Candidate {
    if remain_codes.len() == 0 {
      Candidate::prefect(text)
    } else {
      Candidate::prefix(text, remain_codes)
    }
  }

  pub fn message(&self) -> String {
    let mut res: String = self.text.clone();

    // plain text without color
    for ch in &self.remain_codes {
      res.push(*ch);
    }

    res
  }
}

impl From<Candidate> for Value {
  fn from(v: Candidate) -> Self {
    Value::from(vec![
      (Value::from("text"), Value::from(v.text)),
      (
        Value::from("remain"),
        Value::from(
          v.remain_codes
            .iter()
            .map(|x| Value::from(x.to_string()))
            .collect::<Vec<Value>>(),
        ),
      ),
    ])
  }
}

impl<'a> From<&'a Candidate> for Value {
  fn from(v: &'a Candidate) -> Self {
    Value::from(vec![
      (Value::from("text"), Value::from(v.text.clone())),
      (
        Value::from("remain"),
        Value::from(
          v.remain_codes
            .iter()
            .map(|x| Value::from(x.to_string()))
            .collect::<Vec<Value>>(),
        ),
      ),
    ])
  }
}
