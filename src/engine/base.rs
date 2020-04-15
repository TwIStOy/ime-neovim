pub struct PinyinInitials(String);
pub struct PinyinFinals(String);

pub trait IFilter {
  fn filter(&self, input: &String) -> Vec<String>;

  fn filter_multiple_input(&self, input: &Vec<String>) -> Vec<String> {
    input.iter().map(|s| self.filter(s)).flatten().collect()
  }
}
