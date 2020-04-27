use serde::{Serialize, Deserialize};

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

