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

fn main() -> Result<(), SetLoggerError> {
  // println!("{}", "啊啊啊".chars().count());
  let level = log::LevelFilter::Info;
  let file_path = "/Users/twistoy/.cache/log/ime-neovim.log";

  // Logging to log file.
  let logfile = FileAppender::builder()
    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
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

  let conn = Connector::new();

  conn.recv();

  // {
  //   let mut tr = Trie::<char, String>::new();
  //   tr.insert("aaa".chars().collect::<Vec<char>>().iter(), "1".to_string());
  //   tr.insert("aab".chars().collect::<Vec<char>>().iter(), "1".to_string());
  //   tr.insert("aac".chars().collect::<Vec<char>>().iter(), "1".to_string());
  //   tr.insert("aad".chars().collect::<Vec<char>>().iter(), "1".to_string());
  //   tr.insert("aad".chars().collect::<Vec<char>>().iter(), "2".to_string());
  //   tr.insert(
  //     "aad".chars().collect::<Vec<char>>().iter(),
  //     "啊".to_string(),
  //   );

  //   for line in tr.print_tree() {
  //     println!("{}", line);
  //   }
  // }

  // {
  //   let mut tr = Trie::<String, String>::new();
  //   let mut tmp: Vec<String> = Vec::new();
  //   tmp.push("fuck".to_string());
  //   tmp.push("shit".to_string());
  //   tr.insert(tmp.iter(), "1".to_string());

  //   for line in tr.print_tree() {
  //     println!("{}", line);
  //   }
  // }

  // println!("{:?}", KeyMap::available_keymaps());
  // println!("{:?}", KeyMap::load("小鹤双拼.plist"));

  // let mut codetable = CodeTable::table_file(&"小鹤音形.txt".to_string());
  // let mut ctx = codetable.start_context();

  // ctx.feed('x');
  // println!("{}", serde_json::to_string(&ctx.feed('x')).unwrap());
  Ok(())
}
