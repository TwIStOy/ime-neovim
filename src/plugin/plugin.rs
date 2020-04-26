use super::context_box::ContextBox;
use crate::engine::{BackspaceResult, Candidate, IMEngine, InputContext};
use async_std;
use async_std::io::Stdout;
use async_std::sync::Mutex;
use async_trait::async_trait;
use log::info;
use nvim_rs::{Handler as NeovimHandler, Neovim};
use rmpv::Value;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct PluginManager {
  engine: Arc<Mutex<dyn IMEngine>>,
  contexts: Arc<Mutex<HashMap<String, Arc<Mutex<dyn InputContext>>>>>,
  buffer_box: Arc<Mutex<HashMap<i64, Arc<Mutex<ContextBox>>>>>,
}

#[macro_export]
macro_rules! make_args {
    () => (Vec::new());
    ($($e:expr), +, ) => (make_args![$($e),*]);
    ($($e:expr), +) => {{
      let mut vec = Vec::new();
      $(
        vec.push(Value::from($e));
      )*
      vec
    }}
}

#[macro_export]
macro_rules! vim_dict {
  () => {
    Vec::new()
  };
  ($($key:expr => $value:expr,)+) => {
    vim_dict!($($key => $value),+)
  };
  ($($key:expr => $value:expr),*) => {{
    let mut _res = Vec::<(Value, Value)>::new();
    $(
      _res.push((Value::from($key), Value::from($value)));
    )*
    _res
  }}
}

#[async_trait]
impl NeovimHandler for PluginManager {
  type Writer = Stdout;

  async fn handle_request(
    &self,
    name: String,
    args: Vec<Value>,
    neovim: Neovim<Self::Writer>,
  ) -> Result<Value, Value> {
    match name.as_ref() {
      "start_context" => self.start_context(args, neovim).await,
      "input_char" => self.input_char(args, neovim).await,
      "next_page" => self.next_page(args, neovim).await,
      "previous_page" => self.previous_page(args, neovim).await,
      "backspace" => self.backspace(args, neovim).await,
      "cancel" => self.cancel(args, neovim).await,
      _ => Err(Value::from(format!("no method named: '{}'", name))),
    }
  }
}

impl PluginManager {
  pub fn new(engine: Arc<Mutex<dyn IMEngine>>) -> PluginManager {
    PluginManager {
      engine: engine,
      contexts: Arc::new(Mutex::new(HashMap::new())),
      buffer_box: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  async fn start_context(
    &self,
    _args: Vec<Value>,
    _neovim: Neovim<Stdout>,
  ) -> Result<Value, Value> {
    let uuid = Uuid::new_v4().to_hyphenated().to_string();

    info!("'start_context': generated uuid: {}", uuid);

    let context = self.engine.lock().await.start_context_async();
    self.contexts.lock().await.insert(uuid.clone(), context);

    Ok(Value::from(uuid))
  }

  async fn input_char(&self, _args: Vec<Value>, _neovim: Neovim<Stdout>) -> Result<Value, Value> {
    // args: [context_id, char, bufnr]
    let ((candidates, codes), bufnr) = self._input_char_impl(_args).await?;

    info!(
      "construct ctx_box with candidates: {:?}, codes: {:?}, bufnr: {}",
      candidates, codes, bufnr
    );

    self
      .render_new_buffer_box(bufnr, candidates, codes, &_neovim)
      .await
  }

  async fn _input_char_impl(
    &self,
    args: Vec<Value>,
  ) -> Result<((Vec<Candidate>, Vec<String>), i64), Value> {
    if args.len() < 3 {
      Err(Value::from("expect args but not found"))
    } else {
      let ctx_id = args[0]
        .as_str()
        .ok_or_else(|| Value::from("first parameter should be str"))?;
      let ch = args[1]
        .as_str()
        .ok_or_else(|| Value::from("second parameter should be char"))?;
      if ch.len() < 1 {
        return Err(Value::from("expect char"));
      }

      let bufnr = args[2]
        .as_i64()
        .ok_or_else(|| Value::from("third parameter should be int"))?;

      let ctx = self
        .contexts
        .lock()
        .await
        .get_mut(ctx_id)
        .ok_or_else(|| Value::from("context not exists"))?
        .clone();

      Ok((
        ctx.clone().lock().await.feed(ch.chars().next().unwrap()),
        bufnr,
      ))
    }
  }

  async fn next_page(&self, args: Vec<Value>, neovim: Neovim<Stdout>) -> Result<Value, Value> {
    if args.len() < 1 {
      return Err(Value::from("expect at least 1 argument"));
    }
    let bufnr = args[0]
      .as_i64()
      .ok_or_else(|| Value::from("third parameter should be int"))?;
    match self.buffer_box.lock().await.get(&bufnr) {
      Some(_buf_box) => {
        info!("buf box found!");

        let mut buf_box = _buf_box.lock().await;
        if buf_box.next_page() {
          buf_box.render(&neovim).await?;
        }
      }
      None => {}
    }

    Ok(Value::from("ok"))
  }

  async fn previous_page(&self, args: Vec<Value>, neovim: Neovim<Stdout>) -> Result<Value, Value> {
    if args.len() < 1 {
      return Err(Value::from("expect at least 1 argument"));
    }
    let bufnr = args[0]
      .as_i64()
      .ok_or_else(|| Value::from("third parameter should be int"))?;
    match self.buffer_box.lock().await.get(&bufnr) {
      Some(_buf_box) => {
        info!("buf box found!");

        let mut buf_box = _buf_box.lock().await;
        if buf_box.previous_page() {
          buf_box.render(&neovim).await?;
        }
      }
      None => {}
    }

    Ok(Value::from("ok"))
  }

  async fn backspace(&self, args: Vec<Value>, neovim: Neovim<Stdout>) -> Result<Value, Value> {
    if args.len() < 2 {
      return Err(Value::from("expect at least 2 arguments."));
    }

    let ctx_id = args[0]
      .as_str()
      .ok_or_else(|| Value::from("first parameter should be str"))?;
    let bufnr = args[1]
      .as_i64()
      .ok_or_else(|| Value::from("second parameter should be int"))?;

    let ctx = self
      .contexts
      .lock()
      .await
      .get_mut(ctx_id)
      .ok_or_else(|| Value::from("context not exists"))?
      .clone();

    match ctx.clone().lock().await.backspace() {
      BackspaceResult::Candidates(candidates, codes) => {
        self
          .render_new_buffer_box(bufnr, candidates, codes, &neovim)
          .await
      }
      BackspaceResult::Cancel => self.cancel(args, neovim).await,
    }
  }

  async fn cancel(&self, args: Vec<Value>, neovim: Neovim<Stdout>) -> Result<Value, Value> {
    if args.len() < 2 {
      return Err(Value::from("expect at least 2 arguments."));
    }

    let ctx_id = args[0]
      .as_str()
      .ok_or_else(|| Value::from("first parameter should be str"))?;
    let bufnr = args[1]
      .as_i64()
      .ok_or_else(|| Value::from("second parameter should be int"))?;

    self.contexts.lock().await.remove(ctx_id);
    if let Some(buf_box) = self.buffer_box.lock().await.remove(&bufnr) {
      buf_box.lock().await.close(&neovim).await?;
    }

    Ok(Value::from("canceled "))
  }

  async fn confirm(&self, args: Vec<Value>, neovim: Neovim<Stdout>) -> Result<Value, Value> {
    if args.len() < 3 {
      return Err(Value::from("expect at least 3 arguments"));
    }

    let ctx_id = args[0]
      .as_str()
      .ok_or_else(|| Value::from("first parameter should be str"))?;
    let idx = args[1]
      .as_i64()
      .ok_or_else(|| Value::from("second parameter should be int"))?;
    let bufnr = args[2]
      .as_i64()
      .ok_or_else(|| Value::from("third parameter should be int"))?;

    match self.buffer_box.lock().await.get(&bufnr) {
      Some(_buf_box) => {
        info!("buf box found!");

        let buf_box = _buf_box.lock().await;
        if let Some(txt) = buf_box.confirm(idx) {
          // if ok, cancel it
          self.cancel(make_args![ctx_id, bufnr], neovim).await;

          Ok(Value::from(txt))
        } else {
          Err(Value::from("index out of range"))
        }
      }
      None => Err(Value::from("no buffer box")),
    }
  }

  async fn render_new_buffer_box(
    &self,
    bufnr: i64,
    candidates: Vec<Candidate>,
    codes: Vec<String>,
    neovim: &Neovim<Stdout>,
  ) -> Result<Value, Value> {
    let ctx_box = Arc::new(Mutex::new(ContextBox::new(codes, candidates)));
    match self.buffer_box.lock().await.get(&bufnr) {
      Some(old) => {
        info!("old buffer box found. close it!");
        old.lock().await.close(&neovim).await?;
      }
      None => {}
    }
    self.buffer_box.lock().await.insert(bufnr, ctx_box.clone());

    ctx_box.clone().lock().await.render(&neovim).await?;

    Ok(Value::from("ok"))
  }

  // todo(hawtian): impl this
  async fn register_events() -> Result<Value, Value> {}
}
