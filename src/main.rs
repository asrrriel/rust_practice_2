mod base;
mod tui;
mod util;

use base::entity::*;
use tui::*;

fn main() {
    println!("rust_practice_2 {}",option_env!("CARGO_PKG_VER").unwrap_or("dev_build"));
    let entities : Vec<Enitiy> = Vec::new();

    cli(entities);
}