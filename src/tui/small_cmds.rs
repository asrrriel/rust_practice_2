use crate::base::entity::*;

pub fn cmd_list(entities: &Vec<Entity>) {
    for (i,e) in entities.iter().enumerate() {
        e.describe(Some(i));
    }
}

pub fn cmd_print(entities: &Vec<Entity>, args: &[&str]) -> Result<(),Box<dyn std::error::Error>>{
    if args.len() < 2 {
        println!("Usage: {0} <ID>",args[0]);
    } else{
        let id: usize = args[1].to_string().parse()?;

        match entities.get(id){
            Some(v) => v.describe(Some(id)),
            None => println!("Enity with ID {id} not found.")
        }
    }

    Ok(())
}


pub fn cmd_clear() {
    print!("\x1B[2J\x1B[1;1H"); //ANSI clear and cursore move to 1;1
}