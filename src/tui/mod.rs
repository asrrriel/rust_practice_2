mod primitives;

use crate::base::entity::*;
use primitives::*;


mod small_cmds;
mod file_cmds;
mod cmd_add;
mod cmd_count;
mod cmd_search;
mod cmd_edit;
use small_cmds::*;
use file_cmds::*;
use cmd_add::*;
use cmd_count::*;
use cmd_search::*;
use cmd_edit::*;

fn cmd_help() {
    println!("Available commands:");

    println!("   -add:    adds an entry to the catalog");
    println!("   -clear:  clears the screen");
    println!("   -count:  counts all entries");
    println!("   -edit:   edits an entity");
    println!("   -exit:   quits rp2");
    println!("   -help:   prints this :)");
    println!("   -list:   prints all entries");
    println!("   -load:   loads a catalog");
    println!("   -search: searches the catalog");
    println!("   -print:  prints a specific entity");
    println!("   -write:  saves the catalog");
}

pub fn split_args<'a>(cmd: &'a String) -> Vec<&'a str> {
    let cmd_qouted: Vec<&str> = cmd.split('"')
            .map(|v| { v.trim()})
            .collect();

    return cmd_qouted.iter().enumerate().flat_map(|(i,v)| {
        if i % 2 == 0 {
            if *v == "" { vec![] } else {v.split(' ').collect::<Vec<_>>()}
        } else {
            vec![*v]
        }
    }).collect();
}

pub fn cli(entities: &mut Vec<Entity>) {
    let mut prompt = String::from("rp2>");

    loop {
        let cmd = match input_string(&prompt) {
            Ok(v) => v,
            Err(e) => {
                println!("Failed to read your command: {}",e);
                return;
            }
        };

        let args: Vec<&str> = split_args(&cmd);

        let result =match args[0] {
            "add"   => cmd_add(entities),
            "clear" => Ok(cmd_clear()),
            "count" => Ok(cmd_count(&entities, &args)),
            "edit"  => cmd_edit(entities,&args),
            "exit"  => break,
            "help"  => Ok(cmd_help()),
            "list"  => Ok(cmd_list(&entities)),
            "load"  => cmd_load(entities, &args, &mut prompt),
            "search"=> Ok(cmd_search(&entities, &args)),
            "print" => cmd_print(&entities, &args),
            "write" => cmd_write(&entities, &args, &mut prompt),
            _       => Ok(println!("Nonexistent command \"{0}\", type \"help\" for a list of commands",cmd.trim()))
        };

        match result {
            Err(e) => println!("Error! {e}"),
            _ => {}
        }
    }
}