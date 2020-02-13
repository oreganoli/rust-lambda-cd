#![feature(str_strip)]
#[macro_use]
extern crate log;
mod core;
mod util;
use core::handler;

fn main() {
    pretty_env_logger::init();
    let result = handler();
    dbg!(result);
}
