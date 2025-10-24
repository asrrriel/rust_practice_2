#![feature(vec_try_remove)]
mod base;
mod tui;
mod util;

use base::entity::*;
use tui::*;

fn main() {
    println!("rust_practice_2 {}",option_env!("CARGO_PKG_VER").unwrap_or("dev_build"));
    let mut entities : Vec<Entity> = Vec::new();

    cli(&mut entities);
}