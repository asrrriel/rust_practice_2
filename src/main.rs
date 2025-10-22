mod base;
mod tui;
mod util;

use base::animal::*;
use tui::*;

fn main() {
    let animals : Vec<Animal> = Vec::new();

    cli(animals);
}