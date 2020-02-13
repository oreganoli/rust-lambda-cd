#![feature(str_strip)]
#[macro_use]
extern crate log;
mod core;
mod util;
use crate::core::handler;
use lambda_runtime::lambda;
fn main() {
    pretty_env_logger::init();
    lambda!(handler)
}
