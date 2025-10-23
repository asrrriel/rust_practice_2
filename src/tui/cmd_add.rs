use strum::IntoEnumIterator;

use crate::base::gender::*;
use crate::base::entity::*;
use super::primitives::*;
use std::collections::BTreeMap;

enum GenderHelper<'a>{
    A(Gender<'a>),
    Custom,
    Enough
}

pub fn cmd_add(entities: &mut Vec<Enitiy>) {
    let mut species_m = BTreeMap::<String,Species>::new();

    for s in Species::iter() {
        species_m.insert(s.to_string().to_ascii_lowercase(),s);
    }

    let species = input_one_of(&mut species_m,&"Species[help to list]: ".to_string());

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


    let mut sex_m= BTreeMap::<String,Sex>::new();

    sex_m.insert("male".to_string(), Sex::Male);
    sex_m.insert("female".to_string(), Sex::Female);
    sex_m.insert("intersex".to_string(), Sex::Intersex);

    let sex = input_one_of(&mut sex_m, &"Sex(BIOLOGICAL gender)[help to list]: ".to_string());

    let mut gender_m= BTreeMap::<String,GenderHelper>::new();

    gender_m.insert("male".to_string(), GenderHelper::A(G_MALE.clone()));
    gender_m.insert("female".to_string(), GenderHelper::A(G_FEMALE.clone()));
    gender_m.insert("non-binary".to_string(), GenderHelper::A(G_NON_BINARY.clone()));
    gender_m.insert("custom".to_string(), GenderHelper::Custom);

    let mut genders: Vec<Gender> = Vec::new();

    genders.push(match input_one_of(&mut gender_m, &"Gender[help to list]: ".to_string()) {
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

    gender_m.insert("enough".to_string(), GenderHelper::Enough);

    loop {
        if !gender_m.contains_key("custom") {
            gender_m.insert("custom".to_string(), GenderHelper::Custom);
        }

        let g = match input_one_of(&mut gender_m, &"More genders[help to list/enough to move on]: ".to_string()) {
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