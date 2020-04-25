use super::context_box::ContextBox;
use crate::engine::{Candidate, IMEngine, InputContext};
use async_std;
use async_std::io::Stdout;
use async_trait::async_trait;
use nvim_rs::{Handler as NeovimHandler, Neovim};
use rmpv::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct PluginManager {
  engine: Arc<Mutex<dyn IMEngine>>,
  contexts: Arc<Mutex<HashMap<String, Arc<Mutex<dyn InputContext>>>>>,
  buffer_box: Arc<Mutex<HashMap<u32, Arc<Mutex<ContextBox>>>>>,
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
      _ => Ok(Value::from("v: bool")),
    }
  }
}

impl PluginManager {
  async fn start_context(
    &self,
    _args: Vec<Value>,
    _neovim: Neovim<Stdout>,
  ) -> Result<Value, Value> {
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, "IME-NEOVIM".as_bytes())
      .to_hyphenated()
      .to_string();

    let context: Result<Arc<Mutex<dyn InputContext>>, Value>;
    match self.engine.lock() {
      Ok(engine) => {
        context = Ok(engine.start_context_async());
      }
      Err(_) => return Err(Value::from("failed to start context...")),
    }

    match self.contexts.lock() {
      Ok(mut mp) => {
        mp.insert(uuid.clone(), context?);

        Ok(Value::from(uuid))
      }
      Err(_) => return Err(Value::from("failed to start context...")),
    }
  }

  fn _input_char_impl(&self, args: Vec<Value>) -> Result<Vec<Candidate>, Value> {
    if args.len() < 2 {
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

      Ok(
        self
          .contexts
          .lock()
          .or_else(|_| Err(Value::from("lock failed")))?
          .get_mut(ctx_id)
          .ok_or_else(|| Value::from("context not exists"))?
          .clone()
          .lock()
          .or_else(|_| Err(Value::from("failed to lock input_context")))?
          .feed(ch.chars().next().unwrap()),
      )
    }
  }
}

/*
    match name.as_ref() {
      "file" => {
        let c = neovim.get_current_buf().await.unwrap();
        for _ in 0..1_000_usize {
          let _x = c.get_lines(0, -1, false).await;
        }
        Ok(Value::Nil)
      },
      "buffer" => {
        for _ in 0..10_000_usize {
          let _ = neovim.get_current_buf().await.unwrap();
        }
        Ok(Value::Nil)
      },
      "api" => {
        for _ in 0..1_000_usize {
          let _ = neovim.get_api_info().await.unwrap();
        }
        Ok(Value::Nil)
      },
      _ => Ok(Value::Nil)
    }
  }
}

#[async_std::main]
async fn main() {

  let handler: NeovimHandler = NeovimHandler{};

  let (nvim, io_handler) = create::new_parent(handler).await;

  // Any error should probably be logged, as stderr is not visible to users.
  match io_handler.await {
    Err(err) => {
      if !err.is_reader_error() {
        // One last try, since there wasn't an error with writing to the
        // stream
        nvim
          .err_writeln(&format!("Error: '{}'", err))
          .await
          .unwrap_or_else(|e| {
            // We could inspect this error to see what was happening, and
            // maybe retry, but at this point it's probably best
            // to assume the worst and print a friendly and
            // supportive message to our users
            eprintln!("Well, dang... '{}'", e);
          });
      }

      if !err.is_channel_closed() {
        // Closed channel usually means neovim quit itself, or this plugin was
        // told to quit by closing the channel, so it's not always an error
        // condition.
        eprintln!("Error: '{}'", err);

        let mut source = err.source();

        while let Some(e) = source {
          eprintln!("Caused by: '{}'", e);
          source = e.source();
        }
      }
    }
    Ok(()) => {}
  }
}

 */
