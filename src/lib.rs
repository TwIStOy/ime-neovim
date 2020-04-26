#![feature(get_mut_unchecked)]

pub mod data;
pub mod engine;
pub mod output;
pub mod path;
#[macro_use]
pub mod plugin;
pub mod utility;
pub mod vim;

extern crate async_std;
extern crate dirs;
extern crate log;
extern crate log4rs;
extern crate nvim_rs;
extern crate plist;
extern crate rmpv;
extern crate serde;
extern crate uuid;
