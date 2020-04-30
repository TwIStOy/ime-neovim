use std::path::PathBuf;

pub struct LocalDataPath {
  buf: PathBuf,
}

impl LocalDataPath {
  pub fn new() -> LocalDataPath {
    let mut filepath = dirs::home_dir().unwrap();
    filepath.push(".local");
    filepath.push("share");
    filepath.push("ime-neovim");

    LocalDataPath { buf: filepath }
  }

  pub fn sub(&mut self, name: &str) -> &mut Self {
    self.buf.push(name);

    self
  }

  pub fn file(&self, name: &str) -> PathBuf {
    let mut path = self.buf.clone();
    path.push(name);

    path
  }
}

pub struct LocalConfigPath {
  buf: PathBuf,
}

impl LocalConfigPath {
  pub fn new() -> LocalConfigPath {
    let mut filepath = dirs::home_dir().unwrap();
    filepath.push(".config");
    filepath.push("ime-neovim");

    LocalConfigPath { buf: filepath }
  }

  pub fn sub(&mut self, name: &str) -> &mut Self {
    self.buf.push(name);

    self
  }

  pub fn file(&self, name: &str) -> PathBuf {
    let mut path = self.buf.clone();
    path.push(name);

    path
  }
}
