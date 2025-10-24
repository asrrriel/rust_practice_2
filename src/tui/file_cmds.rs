use crate::{base::entity::Entity, util::serializer::*};
use std::{collections::BTreeMap, fs, io::{self, Read, Write}};
use super::primitives;

pub fn cmd_load(entities: &mut Vec<Entity>, args: &[&str], prompt: &mut String) -> Result<(),Box<dyn std::error::Error>> {
    if args.len() < 2 {
        println!("Usage: {0} <file name>",args[0])
    }

    if !fs::exists(args[1]).unwrap_or(false) {
        return Result::Err(Box::new(io::Error::new(io::ErrorKind::NotFound,"File not found")));
    }

    if !entities.is_empty() {
        let mut map = BTreeMap::<String,bool>::new();

        map.insert("yes".to_string(),true);
        map.insert("no".to_string() ,false);

        if !primitives::input_one_of(&mut map, &"Current database not empty, overwrite it?[yes/no] ".to_string()).unwrap_or(false) {
            return Result::Ok(());
        }
    }

    let mut file = fs::File::open(args[1])?;

    let mut buf = Vec::<u8>::new();

    file.read_to_end(&mut buf)?;

    let mut newdb = deserialize_database(&mut buf)?;

    entities.clear();
    entities.append(&mut newdb);

    prompt.clear();
    prompt.insert_str(0, format!("{0}> ",args[1]).as_str());

    Result::Ok(())
}

pub fn cmd_write(entities: &Vec<Entity>, args: &[&str], prompt: &mut String) -> Result<(),Box<dyn std::error::Error>> {
    if args.len() < 2 {
        println!("Usage: {0} <file name>",args[0])
    }

    if fs::exists(args[1]).unwrap_or(false) {
        let mut map = BTreeMap::<String,bool>::new();

        map.insert("yes".to_string(),true);
        map.insert("no".to_string() ,false);

        if !primitives::input_one_of(&mut map, &"File already exists, overwrite it?[yes/no] ".to_string()).unwrap_or(false) {
            return Result::Err(Box::new(io::Error::new(io::ErrorKind::AlreadyExists,"File already exists")));
        }
    }

    let mut file = fs::File::create(args[1])?;

    let serialized = serialize_database(entities);

    file.write_all(&serialized)?;

    prompt.clear();
    prompt.insert_str(0, format!("{0}> ",args[1]).as_str());

    Ok(())
}