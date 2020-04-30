mod assist_code;
mod codes;
mod pinyin;
mod scheme;

pub use assist_code::AssistCode;
pub use codes::{PinyinCode, PinyinFinals, PinyinInitials};
pub use pinyin::PinyinEngine;
pub use scheme::Scheme;

struct FuzzySyllables(Vec<(PinyinCode, PinyinCode)>);

impl FuzzySyllables {
  pub fn new(v: Vec<(String, String)>) -> FuzzySyllables {
    FuzzySyllables(
      v.iter()
        .map(|(a, b)| (PinyinCode::from(a), PinyinCode::from(b)))
        .collect(),
    )
  }
}
