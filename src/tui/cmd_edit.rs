use std::io;

use crate::{base::{entity::Entity, gender::*}, tui::primitives::input_gender};

fn parse_gender_field<'a>(gender: &str) -> Result<Gender<'a>,Box<dyn std::error::Error>> {
    match gender.to_ascii_lowercase().as_str() {
        "male" => Ok(G_MALE.clone()),
        "female" => Ok(G_FEMALE.clone()),
        "non-binary" => Ok(G_NON_BINARY.clone()),
        "custom" => Ok(input_gender()?),
        _ => Err(Box::new(io::Error::new(io::ErrorKind::NotFound, 
            format!("Nonexistant gender \"{0}\"",gender) 
        )))
    }
}

pub fn cmd_edit(entities: &mut [Entity],args: &[&str]) -> Result<(),Box<dyn std::error::Error>> {
    if args.len() < 3 {
        println!("Usage: {0} <id> age|gender",args[0]);
        return Ok(())
    } 

    let id: usize = args[1].to_string().parse()?;

    if entities.len() < id {
        return Err(Box::new(io::Error::other(
            format!("Nonexistent entity with ID {0}",id)
        ))); 
    }

    let e = &mut entities[id];

    match args[2] {
        "name" => {
            if args.len() < 4 {
                println!("Usage: {0} <id> name \"<new_name>\"",args[0]);
            } else {
                let name = args[3].to_string();
                e.name = name;
            }
        },
        "age" => {
            if args.len() < 4 {
                println!("Usage: {0} <id> age <new_age>",args[0]);
            } else {
                let age: u64 = args[3].parse()?;
                e.age = age;
            }
        },
        "gender" => {
            match args[3] {
                "add" => {
                    if args.len() < 5 {
                        println!("Usage: {0} <id> gender add <new_gender>",args[0]);
                    } else {
                        let gender = parse_gender_field(args[4])?;
                        e.genders.push(gender);
                    }
                },
                "edit" => {
                    if args.len() < 6 {
                        println!("Usage: {0} <id> gender edit <gender_index> <new_gender>",args[0]);
                    } else {
                        let index: usize = args[4].parse()?;
                        let gender = parse_gender_field(args[5])?;

                        if e.genders.len() < index {
                            return Err(Box::new(io::Error::other( 
                                format!("Entity {0} doesnt't have a gender wiht index {1}",id,index)
                            ))); 
                        }

                        e.genders[index] = gender;
                    }
                },
                "remove" => {
                    if args.len() < 6 {
                        println!("Usage: {0} <id> gender remove <gender_index>",args[0]);
                    } else {
                        let index: usize = args[4].parse()?;

                        e.genders.remove(index);
                    }
                }
                _ => {
                    println!("Nonexistent subcommand \"{0}\" for gender editing!",args[3]);
                    return Ok(());
                } 
            }
        },
        _ => {
            println!("Nonexistent subcommand \"{1}\" for {0}!",args[0],args[1]);
            return Ok(());
        } 
    }

    Result::Ok(())
}