use crate::base::{entity::Entity, gender::Sex};

pub fn cmd_search(entities: &Vec<Entity>, args: &Vec<&str>) {
    if args.len() < 2 {
        println!("Usage: {0} name|age|sex|gender|position",args[0]);
        return;
    }

    match args[1] {
        "name" => {
            if args.len() < 3 {
                println!("Usage: {0} name <query>",args[0]);
                return;
            }

            for (i,e) in entities.iter().enumerate() {
                if e.name.starts_with(args[2]) {
                    e.describe(Some(i));
                    return;
                }
            }
        },
        "age" => {
            if args.len() < 3 {
                println!("Usage: {0} age <age>",args[0]);
                return;
            }

            let age: u64 = match args[2].to_string().parse() {
                Ok(v) => v,
                Err(e) => {
                    println!("Failed to parse age: {e}");
                    return;
                }
            };

            for (i,e) in entities.iter().enumerate() {
                if e.age == age {
                    e.describe(Some(i));
                    return;
                }
            }
        },
        "sex" => {
            if args.len() < 3 {
                println!("Usage: {0} name <query>",args[0]);
                return;
            }

            let sex = match Sex::from_string(args[2].to_string()) {
                Some(v) => v,
                None => {
                    println!("Nonexistent sex \"{0}\". Hint: did you mean gender?",args[2]);
                    return;
                }

            };

            for (i,e) in entities.iter().enumerate() {
                if e.sex == sex {
                    e.describe(Some(i));
                    return;
                }
            }
        },
        "gender" => {
            if args.len() < 3 {
                println!("Usage: {0} gender <query>",args[0]);
                return;
            }

            for (i,e) in entities.iter().enumerate() {
                for g in &e.genders {
                    if g.gender_name.to_string().to_ascii_lowercase().starts_with(args[2].to_ascii_lowercase().as_str()) {
                        e.describe(Some(i));
                        return;
                    }
                }
            }
        },
        "position" => {
            if args.len() < 3 {
                println!("Usage: {0} position (<x>,<y>)",args[0]);
                return;
            }

            let pos_str = args[2].to_string();

            if !pos_str.starts_with('(') || !pos_str.ends_with(')'){
                println!("Invalid position syntax! Expected something like (x,y)");
                return;
            }

            let inner = (pos_str.as_str())[1..pos_str.len() - 1].to_string();
            let numbers: Vec<&str> = inner.split(',').collect();

            if numbers.len() != 2 {
                println!("Invalid position syntax! Expected exactly 2 numbers");
                return;
            }

            let x: f32 = match numbers[0].parse() {
                Ok(v) => v,
                Err(e) => {
                    println!("Failed to parse X coordinate: {e}");
                    return;
                }
            };

            let y: f32 = match numbers[1].parse() {
                Ok(v) => v,
                Err(e) => {
                    println!("Failed to parse Y coordinate: {e}");
                    return;
                }
            };

            for (i,e) in entities.iter().enumerate() {
                if e.position == (x,y) {
                    e.describe(Some(i));
                    return;
                }
            }
        },
        _ => {
            println!("Nonexistent subcommand \"{1}\" for {0}!",args[0],args[1]);
            return;
        }
    }
    println!("No match found!");
}