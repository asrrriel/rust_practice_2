use strum::IntoEnumIterator;

use crate::base::animal::{Species, Animal};

fn count_species(animals: &Vec<Animal>,species: Species){
    let mut count: usize = 0; 
 
    for a in animals{
        if a.species == species {
            count += 1;
        } 
    }

    println!("Number of {0}: {1}",species,count);
}


pub fn cmd_count(animals: &Vec<Animal>,args: &Vec<&str>){
    if args.len() < 2 {
        println!("Total amout of animals registered: {0}", animals.len());
    } else {
        match args[1] {
            "species" => {
                if args.len() < 3 {
                    for s in Species::iter() {
                        count_species(&animals, s);
                    }
                } else {
                    match Species::from_string(args[2].to_string()) {
                        Some(v) => count_species(&animals, v),
                        None => println!("Couldn't find species \"{0}\"",args[2])
                    }
                }
            },
            "all" => {
                println!("Total amout of animals registered: {0}", animals.len());

                for s in Species::iter() {
                    count_species(&animals, s);
                }
            }
            _ => println!("illegal subcommand \"{1}\" for {0}!",args[0],args[1])
        }
    }
}