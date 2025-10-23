mod primitives;
mod small_cmds;
mod cmd_add;
mod cmd_count;

use crate::base::entity::*;
use primitives::*;

use small_cmds::*;
use cmd_add::*;
use cmd_count::*;

fn cmd_help() {
    println!("Available commands:");

    println!("   -add:   adds an entry to the registry");
    println!("   -clear: clears the screen");
    println!("   -count: counts all entries");
    println!("   -exit:  quits rp2");
    println!("   -help:  prints this :)");
    println!("   -list:  prints all entries");
    println!("   -print: prints a specific entity");
}

pub fn cli(mut entities: Vec<Enitiy>) {
    loop {
        let cmd = match input_string(&"rp2> ".to_string()) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to read your command: {}",e);
                return;
            }
        };

        let args: Vec<&str> = cmd.split(' ').collect();

        match args[0] {
            "add"   => cmd_add(&mut entities),
            "clear" => cmd_clear(),
            "count" => cmd_count(&entities, &args),
            "exit"  => break,
            "help"  => cmd_help(),
            "list"  => cmd_list(&entities),
            "print" => cmd_print(&entities, &args),
            _       => println!("Nonexistent command \"{0}\", type \"help\" for a list of commands",cmd.trim())
        }
    }
}