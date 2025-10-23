use strum::IntoEnumIterator;

use crate::base::entity::{Species, Entity};

fn count_species(entities: &Vec<Entity>,species: Species){
    let mut count: usize = 0; 
 
    for a in entities{
        if a.species == species {
            count += 1;
        } 
    }

    println!("Number of {0}: {1}",species,count);
}


pub fn cmd_count(entities: &Vec<Entity>,args: &Vec<&str>){
    if args.len() < 2 {
        println!("Total amout of entities registered: {0}", entities.len());
    } else {
        match args[1] {
            "species" => {
                if args.len() < 3 {
                    for s in Species::iter() {
                        count_species(&entities, s);
                    }
                } else {
                    match Species::from_string(args[2].to_string()) {
                        Some(v) => count_species(&entities, v),
                        None => println!("Couldn't find species \"{0}\"",args[2])
                    }
                }
            },
            "all" => {
                println!("Total amout of entities registered: {0}", entities.len());

                for s in Species::iter() {
                    count_species(&entities, s);
                }
            }
            _ => println!("illegal subcommand \"{1}\" for {0}!",args[0],args[1])
        }
    }
}