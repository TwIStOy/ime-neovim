#[derive(Debug)]
pub enum MatchType {
  PerfectMatch,
  PrefixMatch,
  FuzzyMatch,
}

#[derive(Debug)]
pub struct Candidate {
  remain_codes: Vec<char>,
  text: String,
  match_type: MatchType,
}

impl Candidate {
  pub fn new(text: String) -> Candidate {
    Candidate {
      remain_codes: Vec::new(),
      text: text,
      match_type: MatchType::PerfectMatch,
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
