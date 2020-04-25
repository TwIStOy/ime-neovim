use crate::engine::Candidate;
use crate::{make_args, vim_dict};
use async_std::io::Stdout;
use nvim_rs::{error::CallError, rpc::unpack::TryUnpack, Neovim};
use rmpv::Value;
use std::cmp::{max, min};

static CANDIDATE_PER_PAGE: usize = 7;

#[derive(Debug)]
pub struct ContextWindow {
  buf_id: i64,
  win_id: i64,
}

#[derive(Debug)]
pub struct ContextBox {
  candidates: Vec<Candidate>,
  page: usize,
  codes: Vec<String>,
  win_info: Option<ContextWindow>,
}

impl Drop for ContextBox {
  fn drop(&mut self) {}
}

// neovim
// .call(
//   "nvim_buf_set_lines",
//   make_args![self.win_info.as_ref().unwrap().buf_id, 0, -1, true, lines],
// )
// .await??;

macro_rules! call_vim {
    ($neovim:expr, $func:expr, $($args:expr),+) => {
      ($neovim).call($func, make_args![$($args),*]).await??;
    }
}

impl ContextBox {
  pub fn new(codes: Vec<String>, candidates: Vec<Candidate>) -> ContextBox {
    ContextBox {
      codes,
      candidates,
      page: 0,
      win_info: None,
    }
  }

  fn max_page_id(&self) -> usize {
    let mut res = self.candidates.len() / CANDIDATE_PER_PAGE;

    if self.candidates.len() % CANDIDATE_PER_PAGE > 0 {
      res += 1;
    }

    res
  }

  fn candidate_slice(&self) -> &[Candidate] {
    let st = self.page * CANDIDATE_PER_PAGE;
    let ed = min((self.page + 1) * CANDIDATE_PER_PAGE, self.candidates.len());

    &self.candidates[st..ed]
  }

  // @return changed
  pub fn previous_page(&mut self) -> bool {
    if self.page == 0 {
      false
    } else {
      self.page -= 1;

      true
    }
  }

  // @return changed
  pub fn next_page(&mut self) -> bool {
    if self.page < self.max_page_id() {
      self.page += 1;

      true
    } else {
      false
    }
  }

  pub fn box_width(&self) -> usize {
    let mut width: usize = 0;

    // codes will be rendered into "hao'tian"
    width = max(
      self.codes.iter().map(|x| x.len()).sum::<usize>() + (self.codes.len() - 1),
      width,
    );

    let candidates_in_page = self.candidate_slice();
    width = max(
      width,
      // candidate text
      candidates_in_page
        .iter()
        .map(|x| x.text.chars().count() * 2 + x.remain_codes.len())
        .sum::<usize>()
        // seperator
        + (candidates_in_page.len() - 1)
        // number, eg: 1.
        + candidates_in_page.len() * 2,
    );

    width
  }

  async fn create_floating_window(
    &mut self,
    neovim: &Neovim<Stdout>,
  ) -> Result<(), Box<CallError>> {
    if self.win_info.is_some() {
      return Ok(());
    }

    let mut ctx_win = ContextWindow {
      buf_id: 0,
      win_id: 0,
    };
    let opt = vim_dict! {
      "relative" => "cursor",
      "height" => 3,
      "width" => 3,
      "style" => "minimal"
    };

    ctx_win.buf_id = neovim
      .call("nvim_create_buf", make_args![false, true])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))?;
    if ctx_win.buf_id == 0 {
      return Err(Box::new(CallError::WrongValueType(Value::from(
        "failed to create buf",
      ))));
    }
    ctx_win.win_id = neovim
      .call("nvim_open_win", make_args![ctx_win.buf_id, false, opt])
      .await??
      .try_unpack()
      .map_err(|v| Box::new(CallError::WrongValueType(v)))?;

    self.win_info = Some(ctx_win);

    Ok(())
  }

  async fn render_select_box(&self, neovim: &Neovim<Stdout>) -> Result<(), Box<CallError>> {
    if let Some(info) = self.win_info.as_ref() {
      let width = self.box_width();
      let opt = vim_dict![
        "height" => 3 + 2,
        "width" => self.box_width() + 2,
      ];

      // resize window
      call_vim![neovim, "nvim_win_set_config", info.win_id, opt];

      let lines: Vec<Value> = vec![
        Value::from(self.codes.join("'")),
        Value::from("----------------"),
        Value::from(
          self
            .candidates
            .iter()
            .enumerate()
            .map(|(i, candidate)| {
              format!(
                "{}. {}{}",
                i + 1,
                candidate.text,
                candidate.remain_codes.iter().collect::<String>()
              )
            })
            .collect::<Vec<String>>()
            .join(" "),
        ),
      ];

      neovim
        .call(
          "nvim_buf_set_lines",
          make_args![self.win_info.as_ref().unwrap().buf_id, 0, -1, true, lines],
        )
        .await??;

      Ok(())
    } else {
      Err(Box::new(CallError::WrongValueType(Value::from(
        "window has not been created",
      ))))
    }
  }

  pub async fn render(&mut self, neovim: &Neovim<Stdout>) {
    let candidates_this_page = self.candidate_slice();

    if self.win_info.is_none() {
      self.create_floating_window(neovim).await;
    }

    self.render_select_box(neovim).await;
  }
}
