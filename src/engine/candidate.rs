use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MatchType {
  PerfectMatch,
  PrefixMatch,
  FuzzyMatch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
  remain_codes: Vec<char>,
  text: String,
  match_type: MatchType,
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
