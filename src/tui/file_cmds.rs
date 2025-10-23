use crate::base::entity::Enitiy;


pub fn cmd_load(_entities: &Vec<Enitiy>, args: &Vec<&str>, prompt: &mut String) {
    if args.len() < 2 {
        println!("Usage: {0} <file name>",args[0])
    }

    prompt.clear();
    prompt.insert_str(0, format!("{0}> ",args[1]).as_str());


    println!("Load is unimplemented, it only changes the prompt!");
}

pub fn cmd_write(_entities: &Vec<Enitiy>, args: &Vec<&str>, _prompt: &mut String) {
    if args.len() < 2 {
        println!("Usage: {0} <file name>",args[0])
    }

    println!("Write is unimplemented!");
}