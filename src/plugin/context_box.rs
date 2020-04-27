use crate::engine::Candidate;
use crate::{make_args, vim_dict};
use async_std::io::Stdout;
use log::info;
use nvim_rs::{neovim_api, neovim_api_manual, rpc::unpack::TryUnpack, Buffer, Neovim, Window};
use rmpv::Value;
use std::cmp::{max, min};

pub static CANDIDATE_PER_PAGE: usize = 7;

struct CornerStyle {
  lu: &'static str,
  ru: &'static str,
  lb: &'static str,
  rb: &'static str,
}
struct BorderStyle {
  corner: CornerStyle,
  simple_left: &'static str,
  simple_right: &'static str,
  sep_left: &'static str,
  sep_right: &'static str,
  simple_up: &'static str,
  simple_bottom: &'static str,
  seperator: &'static str,
}
static BORDER: BorderStyle = BorderStyle {
  corner: CornerStyle {
    lu: "┌",
    ru: "┐",
    lb: "└",
    rb: "┘",
  },
  simple_left: "│",
  simple_right: "│",
  sep_left: "├",
  sep_right: "┤",
  simple_up: "─",
  simple_bottom: "─",
  seperator: "─",
};

pub struct ContextWindow {
  buffer: Buffer<Stdout>,
  window: Window<Stdout>,
}

pub struct ContextBox {
  candidates: Vec<Candidate>,
  page: usize,
  codes: Vec<String>,
  win_info: Option<ContextWindow>,
}

macro_rules! call_vim {
    ($neovim:expr, $func:expr, $($args:expr),+) => {
      ($neovim).call($func, make_args![$($args),*]).await.map_err(|_| Value::from(format!("call neovim function {} failed", $func)))??
    }
}

macro_rules! eval_vim {
  ($neovim:expr, $args:expr) => {
    ($neovim)
      .call("nvim_eval", make_args![$args])
      .await
      .map_err(|_| Value::from(format!("call neovim eval '{}' failed", $args)))??
  };
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

  pub async fn close(&self, neovim: &Neovim<Stdout>) -> Result<(), Value> {
    match &self.win_info {
      Some(info) => {
        // info!("close ctx_box: {:?}", info);
        // call_vim![neovim, "nvim_win_close", info.win_id, true];
        info
          .window
          .close(true)
          .await
          .map_err(|_| Value::from("close window failed"))?;
      }
      None => {}
    }

    Ok(())
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

  pub fn confirm(&self, idx: i64) -> Option<String> {
    let candidates = self.candidate_slice();
    if candidates.len() == 0 {
      Some(self.codes.join(""))
    } else {
      if candidates.len() >= idx as usize {
        Some(candidates[idx as usize - 1].text.clone())
      } else {
        None
      }
    }
  }

  async fn create_floating_window(&mut self, neovim: &Neovim<Stdout>) -> Result<(), Value> {
    info!("create floating window");

    if self.win_info.is_some() {
      return Ok(());
    }

    let opt = vim_dict! {
      "relative" => "cursor",
      "height" => 3 + 2,
      "width" => 3 + 2,
      "style" => "minimal",
      "row" => 1,
      "col" => 1,
    };

    let buffer = neovim
      .create_buf(false, true)
      .await
      .map_err(|_| Value::from("create buffer failed"))?;
    // ctx_win.buf_id = eval_vim![neovim, "bufadd('ime-selector')"].try_unpack()?;
    // if ctx_win.buf_id == 0 {
    //   return Err(Value::from("failed to create buf"));
    // }
    // info!("float buffer created with id: {}", ctx_win.buf_id);
    let window = neovim
      .open_win(&buffer, false, opt)
      .await
      .map_err(|_| Value::from("open float win failed"))?;
    // ctx_win.win_id = call_vim![neovim, "nvim_open_win", ctx_win.buf_id, false, opt].try_unpack()?;
    // info!("fuck");
    self.win_info = Some(ContextWindow { buffer, window });

    Ok(())
  }

  async fn render_select_box(&self, _neovim: &Neovim<Stdout>) -> Result<(), Value> {
    if let Some(info) = &self.win_info {
      let candidates = self.candidate_slice();
      info!("candidates this page: {:?}", candidates);

      let lines: Vec<String> = vec![
        self.codes.join("'"),
        "--".to_string(),
        candidates
          .iter()
          .enumerate()
          .map(|(i, candidate)| {
            format!(
              "{}.{}{}",
              i + 1,
              candidate.text,
              candidate.remain_codes.iter().collect::<String>()
            )
          })
          .collect::<Vec<String>>()
          .join("  "),
      ];
      info!("lines before bordered: {:?}", lines);

      let mut width = 0;
      for s in &lines {
        width = max(
          s.chars().map(|ch| if ch.is_ascii() { 1 } else { 2 }).sum(),
          width,
        );
      }

      // resize window
      let opt = vim_dict![
        "height" => 3 + 2,
        "width" => width + 2,
      ];
      info!("render selection box to ({}, {})", 5, width + 2);
      info
        .window
        .set_config(opt)
        .await
        .map_err(|_| Value::from("update window config failed"))?;
      info!("update window size to ({}, {})", 5, width + 2);

      let bordered_text = Self::make_bordered_text(lines, width);
      info!("bordered text: {:?}", bordered_text);

      info
        .buffer
        .set_lines(0, -1, false, bordered_text)
        .await
        .map_err(|_| Value::from("set lines failed"))?;
      Ok(())
    } else {
      Err(Value::from("window has not been created"))
    }
  }

  pub async fn render(&mut self, neovim: &Neovim<Stdout>) -> Result<(), Value> {
    // let candidates_this_page = self.candidate_slice();

    if self.win_info.is_none() {
      self.create_floating_window(neovim).await?;
    }

    self.render_select_box(neovim).await?;

    Ok(())
  }

  fn make_bordered_text(text: Vec<String>, width: usize) -> Vec<String> {
    let mut res = vec![vec![
      BORDER.corner.lu,
      &BORDER.simple_up.repeat(width),
      BORDER.corner.ru,
    ]
    .join("")];

    for s in text {
      if s != "--" {
        res.push(
          vec![
            BORDER.simple_left,
            &s,
            &" ".repeat(width - Self::eval_length(&s)),
            BORDER.simple_right,
          ]
          .join(""),
        );
      } else {
        res.push(
          vec![
            BORDER.sep_left,
            &BORDER.seperator.repeat(width),
            BORDER.sep_right,
          ]
          .join(""),
        );
      }
    }

    res.push(
      vec![
        BORDER.corner.lb,
        &BORDER.simple_bottom.repeat(width),
        BORDER.corner.rb,
      ]
      .join(""),
    );

    res
  }

  fn eval_length(s: &String) -> usize {
    s.chars().map(|ch| if ch.is_ascii() { 1 } else { 2 }).sum()
  }
}
