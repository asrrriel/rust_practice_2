mod base;
mod tui;
mod util;

use base::animal::*;
use tui::*;

fn main() {
    println!("rust_practice_2 {}",option_env!("CARGO_PKG_VER").unwrap_or("dev_build"));
    let animals : Vec<Animal> = Vec::new();

    cli(animals);
}