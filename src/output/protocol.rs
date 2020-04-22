use crate::engine::candidate::Candidate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
enum InputEvent {
  InputChar(char),
  Backspace,
  Cancel,
}

#[derive(Serialize, Deserialize)]
struct InputRequest {
  context_id: String,
  event: InputEvent,
}

#[derive(Serialize, Deserialize)]
struct InputResponse {
  context_id: String,
  candidates: Vec<Candidate>,
}
