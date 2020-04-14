pub struct Candidate {
  remain_codes: Vec<char>,
  text: String,
}

impl Candidate {
  pub fn message(&self) -> String {
    let mut res: String = self.text.clone();

    // plain text without color
    for ch in &self.remain_codes {
      res.push(*ch);
    }

    res
  }
}
