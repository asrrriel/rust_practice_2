use crate::base::gender::*;
use crate::base::animal::*;
use super::primitives::*;
use std::collections::HashMap;

enum GenderHelper<'a>{
    A(Gender<'a>),
    Custom,
    Enough
}

pub fn cmd_add(animals: &mut Vec<Animal>) {
    let mut species_hm = HashMap::<String,Species>::new();

    species_hm.insert("cat".to_string()      ,Species::Cat);
    species_hm.insert("cattle".to_string()   ,Species::Cattle);
    species_hm.insert("dog".to_string()      ,Species::Dog);
    species_hm.insert("dragon".to_string()   ,Species::Dragon);
    species_hm.insert("elephant".to_string() ,Species::Elephant);
    species_hm.insert("fox".to_string()      ,Species::Fox);
    species_hm.insert("goat".to_string()     ,Species::Goat);
    species_hm.insert("human".to_string()    ,Species::Human);
    species_hm.insert("jay".to_string()      ,Species::Jay);
    species_hm.insert("lion".to_string()     ,Species::Lion);
    species_hm.insert("lizard".to_string()   ,Species::Lizard);
    species_hm.insert("ox".to_string()       ,Species::Ox);
    species_hm.insert("pig".to_string()      ,Species::Pig);
    species_hm.insert("shark".to_string()    ,Species::Shark);
    species_hm.insert("sheep".to_string()    ,Species::Sheep);
    species_hm.insert("sparrow".to_string()  ,Species::Sparrow);
    species_hm.insert("wolf".to_string()     ,Species::Wolf);
    species_hm.insert("zebra".to_string()    ,Species::Zebra);


    let species = input_one_of(&mut species_hm,&"Species[help to list]: ".to_string());

    let age; 
    loop {
        match input_int(&"Age[whole number]: ".to_string()) {
            Ok(v) => {
                age = v;
                break;
            },
            Err(e) => {
                println!("Invalid number: {}",e);
            } 
        };
    }

    let name = match input_string(&"Name[any string]: ".to_string()) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to take your input: {}",e);
            return;
        } 
    };


    let mut sex_hm= HashMap::<String,Sex>::new();

    sex_hm.insert("male".to_string(), Sex::Male);
    sex_hm.insert("female".to_string(), Sex::Female);
    sex_hm.insert("intersex".to_string(), Sex::Intersex);

    let sex = input_one_of(&mut sex_hm, &"Sex(BIOLOGICAL gender)[male/female/intersex]: ".to_string());

    let mut gender_hm= HashMap::<String,GenderHelper>::new();

    gender_hm.insert("male".to_string(), GenderHelper::A(G_MALE.clone()));
    gender_hm.insert("female".to_string(), GenderHelper::A(G_FEMALE.clone()));
    gender_hm.insert("non-binary".to_string(), GenderHelper::A(G_NON_BINARY.clone()));
    gender_hm.insert("custom".to_string(), GenderHelper::Custom);

    let mut genders: Vec<Gender> = Vec::new();

    genders.push(match input_one_of(&mut gender_hm, &"Gender[male/female/non-binary/custom]: ".to_string()) {
        GenderHelper::A(g) => g,
        GenderHelper::Custom => {
            match input_gender() {
                Ok(v) => v,
                Err(e) => {
                    println!("Failed to take your input: {}",e);
                    return;
                }
            }
        } 
        GenderHelper::Enough => {
            panic!("Impossible code path triggered");
        }
    });

    gender_hm.insert("enough".to_string(), GenderHelper::Enough);

    loop {
        if !gender_hm.contains_key("custom") {
            gender_hm.insert("custom".to_string(), GenderHelper::Custom);
        }

        let g = match input_one_of(&mut gender_hm, &"More genders[help to list/enough to move on]: ".to_string()) {
            GenderHelper::A(g) => g,
            GenderHelper::Custom => {
                match input_gender() {
                    Ok(v) => v,
                    Err(e) => {
                        println!("Failed to take your input: {}",e);
                        return;
                    }
                }
            } 
            GenderHelper::Enough => {
                break;
            }
        };

        genders.push(g);
    }

    animals.push(Animal { 
        species: species, 
        age: age as u64,
        name: name, 
        sex: sex, 
        genders: genders,
        position: (0.,0.) }
    );
}