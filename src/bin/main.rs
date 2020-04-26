use async_std;
use async_std::sync::Mutex;
use ime::engine::codetable::code_table::CodeTable;
use ime::plugin::PluginManager;
use ime::vim::connector::Connector;
use log::{error, info, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::{
  append::{
    console::{ConsoleAppender, Target},
    file::FileAppender,
  },
  config::{Appender, Config, Root},
  encode::pattern::PatternEncoder,
  filter::threshold::ThresholdFilter,
};
use nvim_rs::create::async_std as create;
use std::sync::Arc;

#[async_std::main]
async fn main() -> Result<(), SetLoggerError> {
  // println!("{}", "啊啊啊".chars().count());
  let file_path = "/Users/twistoy/.cache/log/ime-neovim.log";

  // Logging to log file.
  let logfile = FileAppender::builder()
    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    .encoder(Box::new(PatternEncoder::new(
      "{l}[{T}:{I}] [{M}:{L}] {m}\n",
    )))
    .build(file_path)
    .unwrap();

  // Log Trace level output to file where trace is the default level
  // and the programmatically specified level to stderr.
  let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(
      Root::builder()
        .appender("logfile")
        .build(LevelFilter::Trace),
    )
    .unwrap();

  // Use this to change log levels at runtime.
  // This means you can change the default log level to trace
  // if you are trying to debug an issue and need more logs on then turn it off
  // once you are done.
  let _handle = log4rs::init_config(config)?;

  let handler = PluginManager::new(Arc::new(Mutex::new(CodeTable::table_file("小鹤音形.txt"))));
  let (nvim, io_handler) = create::new_parent(handler).await;

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
            error!("Well, dang... '{}'", e);
          });
      }

      if !err.is_channel_closed() {
        // Closed channel usually means neovim quit itself, or this plugin was
        // told to quit by closing the channel, so it's not always an error
        // condition.
        error!("Error: '{}'", err);

        // let mut source = err.source();

        // while let Some(e) = source {
        //   eprintln!("Caused by: '{}'", e);
        //   source = e.source();
        // }
      }
    }
    Ok(()) => {}
  }

  Ok(())
}
