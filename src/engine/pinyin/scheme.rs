use super::codes::{PinyinCode, PinyinFinals, PinyinInitials};
use crate::path::LocalDataPath;
use plist;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct Scheme {
  initials: Arc<HashMap<String, Vec<PinyinInitials>>>,
  finals: Arc<HashMap<String, Vec<PinyinFinals>>>,
  special: Arc<HashMap<String, Vec<PinyinCode>>>,
  version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyMapProtocol {
  Sheng: HashMap<String, String>,
  Yun: HashMap<String, String>,
  Special: HashMap<String, String>,
  Vesion: String,
}

impl Scheme {
  pub fn new(filename: &str) -> Scheme {
    let filepath = LocalDataPath::new().sub("keymap").file(filename);

    let keymap: KeyMapProtocol = plist::from_file(filepath.as_path()).expect("parse plit failed");

    let (initials, finals, special) =
      Scheme::parse_keymap(keymap.Sheng, keymap.Yun, keymap.Special);

    Scheme {
      initials: Arc::new(initials),
      finals: Arc::new(finals),
      special: Arc::new(special),
      version: keymap.Vesion,
    }
  }

  pub fn keycodes(&self) -> HashSet<char> {
    let mut res = HashSet::<char>::new();

    for (key, _) in self.initials.as_ref() {
      for ch in key.chars() {
        res.insert(ch);
      }
    }
    for (key, _) in self.finals.as_ref() {
      for ch in key.chars() {
        res.insert(ch);
      }
    }
    for (key, _) in self.special.as_ref() {
      for ch in key.chars() {
        res.insert(ch);
      }
    }

    res
  }

  fn parse_keymap(
    sheng: HashMap<String, String>,
    yun: HashMap<String, String>,
    special: HashMap<String, String>,
  ) -> (
    HashMap<String, Vec<PinyinInitials>>,
    HashMap<String, Vec<PinyinFinals>>,
    HashMap<String, Vec<PinyinCode>>,
  ) {
    let mut initials = HashMap::<String, Vec<PinyinInitials>>::new();
    let mut finals = HashMap::<String, Vec<PinyinFinals>>::new();
    let mut codes = HashMap::<String, Vec<PinyinCode>>::new();

    for (keycode, value) in sheng {
      if let Some(_x) = Scheme::parse_pinyin_code(&value) {
        for x in _x {
          match x {
            PinyinCode::Initials(initial_code) => {
              initials.entry(keycode.clone()).or_default().push(initial_code);
            }
            PinyinCode::Finals(final_code) => {
              finals.entry(keycode.clone()).or_default().push(final_code);
            }
          }
        }
      }
    }

    (initials, finals, codes)
  }

  fn parse_pinyin_code(code: &str) -> Option<Vec<PinyinCode>> {
    let mut res: Vec<PinyinCode> = Vec::new();
    for part in code.split("|").collect::<Vec<&str>>() {
      res.push(match part.to_lowercase().as_str() {
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
        _ => return None,
      });
    }

    Some(res)
  }
}
