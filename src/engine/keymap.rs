use crate::path::LocalDataPath;
use dirs;
use plist;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

type KeyCodeTranslateDict = HashMap<char, String>;
type KeyCodeTranslation = HashMap<char, Vec<String>>;

#[derive(Deserialize, Serialize, Debug)]
struct KeyMapProtocol {
  Sheng: KeyCodeTranslateDict,
  Yun: KeyCodeTranslateDict,
  Special: KeyCodeTranslateDict,
  Version: String,
}

/// overwrite keycode to a keycode list
#[derive(Default, Debug)]
pub struct KeyMap {
  Sheng: KeyCodeTranslation,
  Yun: KeyCodeTranslation,
  Special: KeyCodeTranslation,
}

impl KeyMap {
  fn convert(dict: &KeyCodeTranslateDict) -> KeyCodeTranslation {
    let mut res = KeyCodeTranslation::new();

    for (k, v) in dict {
      let parts: Vec<&str> = v.as_str().split("|").collect();

      for n in parts {
        res.entry(*k).or_insert(Vec::new()).push(n.to_string());
      }
    }

    res
  }

  pub fn load(filename: &str) -> KeyMap {
    let filepath = LocalDataPath::new().sub("keymap").file(filename);

    let keymap: KeyMapProtocol = plist::from_file(filepath.as_path()).expect("parse plist failed");

    KeyMap {
      Sheng: KeyMap::convert(&keymap.Sheng),
      Yun: KeyMap::convert(&keymap.Yun),
      Special: KeyMap::convert(&keymap.Special),
    }
  }

  pub fn available_keymaps() -> Vec<String> {
    let mut res = Vec::<String>::new();

    match dirs::home_dir() {
      Some(mut home) => {
        home.push(".local");
        home.push("share");
        home.push("ime-neovim");
        home.push("keymap");

        let keymap_dir =
          fs::read_dir(home.as_path()).expect(&format!("failed to locate {}", home.display()));
        for entry in keymap_dir {
          match entry {
            Ok(file) => {
              res.push(file.file_name().into_string().unwrap());
            }
            Err(..) => {}
          }
        }
      }
      None => {
        panic!("failed to get home directory.");
      }
    }

    res
  }
}
