use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PinyinInitials {
  B,
  P,
  M,
  F,
  D,
  T,
  N,
  L,
  G,
  K,
  H,
  J,
  Q,
  X,
  ZH,
  CH,
  SH,
  R,
  C,
  S,
  Y,
  W,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PinyinFinals {
  A,
  O,
  E,
  I,
  U,
  V,
  AI,
  EI,
  UI,
  AO,
  OU,
  IU,
  IE,
  VE,
  ER,
  AN,
  EN,
  IN,
  UN,
  VN,
  ANG,
  ENG,
  ING,
  ONG,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PinyinCode {
  Initials(PinyinInitials),
  Finals(PinyinFinals),
}

impl From<String> for PinyinCode {
  fn from(s: String) -> PinyinCode {
    match s.to_lowercase().as_str() {
      "b" => (PinyinCode::Initials(PinyinInitials::B)),
      "p" => (PinyinCode::Initials(PinyinInitials::P)),
      "m" => (PinyinCode::Initials(PinyinInitials::M)),
      "f" => (PinyinCode::Initials(PinyinInitials::F)),
      "d" => (PinyinCode::Initials(PinyinInitials::D)),
      "t" => (PinyinCode::Initials(PinyinInitials::T)),
      "n" => (PinyinCode::Initials(PinyinInitials::N)),
      "l" => (PinyinCode::Initials(PinyinInitials::L)),
      "g" => (PinyinCode::Initials(PinyinInitials::G)),
      "k" => (PinyinCode::Initials(PinyinInitials::K)),
      "h" => (PinyinCode::Initials(PinyinInitials::H)),
      "j" => (PinyinCode::Initials(PinyinInitials::J)),
      "q" => (PinyinCode::Initials(PinyinInitials::Q)),
      "x" => (PinyinCode::Initials(PinyinInitials::X)),
      "zh" => (PinyinCode::Initials(PinyinInitials::ZH)),
      "ch" => (PinyinCode::Initials(PinyinInitials::CH)),
      "sh" => (PinyinCode::Initials(PinyinInitials::SH)),
      "r" => (PinyinCode::Initials(PinyinInitials::R)),
      "c" => (PinyinCode::Initials(PinyinInitials::C)),
      "s" => (PinyinCode::Initials(PinyinInitials::S)),
      "y" => (PinyinCode::Initials(PinyinInitials::Y)),
      "w" => (PinyinCode::Initials(PinyinInitials::W)),
      "a" => (PinyinCode::Finals(PinyinFinals::A)),
      "o" => (PinyinCode::Finals(PinyinFinals::O)),
      "e" => (PinyinCode::Finals(PinyinFinals::E)),
      "i" => (PinyinCode::Finals(PinyinFinals::I)),
      "u" => (PinyinCode::Finals(PinyinFinals::U)),
      "v" => (PinyinCode::Finals(PinyinFinals::V)),
      "ai" => (PinyinCode::Finals(PinyinFinals::AI)),
      "ei" => (PinyinCode::Finals(PinyinFinals::EI)),
      "ui" => (PinyinCode::Finals(PinyinFinals::UI)),
      "ao" => (PinyinCode::Finals(PinyinFinals::AO)),
      "ou" => (PinyinCode::Finals(PinyinFinals::OU)),
      "iu" => (PinyinCode::Finals(PinyinFinals::IU)),
      "ie" => (PinyinCode::Finals(PinyinFinals::IE)),
      "ve" => (PinyinCode::Finals(PinyinFinals::VE)),
      "er" => (PinyinCode::Finals(PinyinFinals::ER)),
      "an" => (PinyinCode::Finals(PinyinFinals::AN)),
      "en" => (PinyinCode::Finals(PinyinFinals::EN)),
      "in" => (PinyinCode::Finals(PinyinFinals::IN)),
      "un" => (PinyinCode::Finals(PinyinFinals::UN)),
      "vn" => (PinyinCode::Finals(PinyinFinals::VN)),
      "ang" => (PinyinCode::Finals(PinyinFinals::ANG)),
      "eng" => (PinyinCode::Finals(PinyinFinals::ENG)),
      "ing" => (PinyinCode::Finals(PinyinFinals::ING)),
      "ong" => (PinyinCode::Finals(PinyinFinals::ONG)),
      _ => panic!("load pinyin code failed, unknown {}", s.to_lowercase()),
    }
  }
}

impl From<&String> for PinyinCode {
  fn from(s: &String) -> PinyinCode {
    PinyinCode::from(s.to_lowercase())
  }
}
