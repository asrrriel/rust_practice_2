mod primitives;
mod small_cmds;
mod cmd_add;

use crate::base::animal::*;
use primitives::*;

use small_cmds::*;
use cmd_add::*;

pub fn cli(mut animals: Vec<Animal>) {
    loop {
        let cmd = match input_string(&"zoo> ".to_string()) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to read your command: {}",e);
                return;
            }
        };

        match cmd.as_str() {
            "add"   => cmd_add(&mut animals),
            "list" => cmd_list(&animals),
            "exit"  => break,
            _   => println!("Was that a typo or are you high? seriously, \"{0}\"?",cmd.trim())

        }
    }
}