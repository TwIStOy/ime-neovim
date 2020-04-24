use crate::engine::{IMEngine, InputContext};
use async_std;
use async_std::io::Stdout;
use async_trait::async_trait;
use nvim_rs::{Handler as NeovimHandler, Neovim};
use rmpv::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
struct PluginManager {
  engine: Arc<Mutex<dyn IMEngine>>,
  contexts: Arc<Mutex<HashMap<String, Arc<Mutex<dyn InputContext>>>>>,
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
  async fn start_context(&self, args: Vec<Value>, neovim: Neovim<Stdout>) -> Result<Value, Value> {
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_DNS, "IME-NEOVIM".as_bytes())
      .to_hyphenated()
      .to_string();

    let mut context: Result<Arc<Mutex<dyn InputContext>>, Value> =
      Err(Value::from("failed to start context..."));
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
