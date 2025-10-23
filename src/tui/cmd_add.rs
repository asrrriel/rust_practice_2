use strum::IntoEnumIterator;

use crate::base::gender::*;
use crate::base::entity::*;
use super::primitives::*;
use std::collections::HashMap;

enum GenderHelper<'a>{
    A(Gender<'a>),
    Custom,
    Enough
}

pub fn cmd_add(entities: &mut Vec<Enitiy>) {
    let mut species_hm = HashMap::<String,Species>::new();

    for s in Species::iter() {
        species_hm.insert(s.to_string().to_ascii_lowercase(),s);
    }

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

    let sex = input_one_of(&mut sex_hm, &"Sex(BIOLOGICAL gender)[help to list]: ".to_string());

    let mut gender_hm= HashMap::<String,GenderHelper>::new();

    gender_hm.insert("male".to_string(), GenderHelper::A(G_MALE.clone()));
    gender_hm.insert("female".to_string(), GenderHelper::A(G_FEMALE.clone()));
    gender_hm.insert("non-binary".to_string(), GenderHelper::A(G_NON_BINARY.clone()));
    gender_hm.insert("custom".to_string(), GenderHelper::Custom);

    let mut genders: Vec<Gender> = Vec::new();

    genders.push(match input_one_of(&mut gender_hm, &"Gender[help to list]: ".to_string()) {
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
            panic!("Impossible code path");
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

    entities.push(Enitiy { 
        species: species, 
        age: age as u64,
        name: name, 
        sex: sex, 
        genders: genders,
        position: (0.,0.) }
    );
}