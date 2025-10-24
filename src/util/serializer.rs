use std::io;

use crate::{base::{entity::*, gender::*}, util::flexistring::Flexistring};
use crc32fast;

pub fn serialize_database(database: &Vec<Entity>) -> Vec<u8>{
    let mut vec = Vec::<u8>::new();

    vec.extend_from_slice(&[b'R',b'P',b'2',0xAA]);                  // magic number
    vec.extend_from_slice(&[0,0,0,0]);                              // space for checksum
    vec.extend_from_slice(&(database.len() as u32).to_le_bytes());  // database length

    for e in database{
        let mut tempvec = Vec::<u8>::new();

        tempvec.extend_from_slice(&(e.species.clone() as u8).to_le_bytes()); // speceis
        tempvec.extend_from_slice(&e.age.to_le_bytes());                     // age
        tempvec.extend_from_slice(&(e.name.len() as u32).to_le_bytes());     // length of name
        tempvec.extend_from_slice(e.name.as_bytes());                        // name
        tempvec.extend_from_slice(&(e.sex.clone() as u8).to_le_bytes());     // sex
        tempvec.extend_from_slice(&(e.genders.len() as u32).to_le_bytes());  // number of genders

        for g in &e.genders{
            tempvec.extend_from_slice(&(g.gender_name.to_string().len() as u32).to_le_bytes()); // length of gender name
            tempvec.extend_from_slice(g.gender_name.to_string().as_bytes());                    // gender name

            tempvec.extend_from_slice(&(g.subj_pronoun.to_string().len() as u8).to_le_bytes()); // length of subject pronoun
            tempvec.extend_from_slice(g.subj_pronoun.to_string().as_bytes());                   // subject pronoun

            tempvec.extend_from_slice(&(g.obj_pronoun.to_string().len() as u8).to_le_bytes());  // length of object pronoun
            tempvec.extend_from_slice(g.obj_pronoun.to_string().as_bytes());                    // object pronoun

            tempvec.extend_from_slice(&[
                if g.pluralized { 0x55 } else { 0xAA }
            ]);
        }

        tempvec.extend_from_slice(&(e.position.0 as u32).to_le_bytes());     // X position
        tempvec.extend_from_slice(&(e.position.1 as u32).to_le_bytes());     // Y position

        vec.extend_from_slice(&(tempvec.len() as u32).to_le_bytes());
        vec.extend_from_slice(tempvec.as_slice());
    }

    let hash = crc32fast::hash(vec.as_slice()).to_le_bytes();
    vec[4..8].copy_from_slice(&hash);

    vec
}

fn get_bytes<const N: usize>(data: &[u8],from: usize) -> Result<[u8;N],Box<dyn std::error::Error>> {
    if data.len() < from + N {
        return Result::Err(Box::new(io::Error::new(io::ErrorKind::InvalidData,"Data too short")));
    }

    Ok(data[from..from + N].try_into()?)
}

fn get_bytes_arr(data: &[u8],from: usize,to: usize) -> Result<&[u8],Box<dyn std::error::Error>> {
    if data.len() < to {
        return Result::Err(Box::new(io::Error::new(io::ErrorKind::InvalidData,"Data too short")));
    }

    Ok(data[from..to].try_into()?)
}

pub fn deserialize_database<'a>(data: &mut [u8]) -> Result<Vec<Entity<'a>>,Box<dyn std::error::Error>>{
    if get_bytes(data,0)? != [b'R',b'P',b'2',0xAA] {
        return Result::Err(Box::new(io::Error::new(io::ErrorKind::InvalidData,"No magic number")));
    }

    let crc32 = u32::from_le_bytes(get_bytes(data,4)?);

    data[4..8].fill(0);

    if crc32 != crc32fast::hash(data) {
        return Result::Err(Box::new(io::Error::new(io::ErrorKind::InvalidData,"CC32 mismatch")));
    }

    let num_entities = u32::from_le_bytes(get_bytes(data,8)?);

    let mut entities = Vec::<Entity>::new();

    let mut cur_byte = 12;

    for _ in 0..num_entities {
        let ent_size = u32::from_le_bytes(get_bytes(data,cur_byte)?);
        cur_byte += 4;

        let old_cur = cur_byte;

        let mut ent = Entity{
            ..Entity::default()
        };

        ent.species = Species::from_repr(
                 u8::from_le_bytes(get_bytes(data,cur_byte)?).into()
            ).ok_or(
                io::Error::other("Failed to deserialize species")
            )?;
        cur_byte += 1;

        ent.age = u64::from_le_bytes(get_bytes(data,cur_byte)?);
        cur_byte += 8;

        let name_len = u32::from_le_bytes(get_bytes(data,cur_byte)?);
        cur_byte += 4;

        ent.name = String::from_utf8_lossy(get_bytes_arr(data,cur_byte,cur_byte+name_len as usize)?).to_string();
        cur_byte += name_len as usize;

        ent.sex = Sex::from_repr(
                 u8::from_le_bytes(get_bytes(data,cur_byte)?).into()
            ).ok_or(
                io::Error::other("Failed to deserialize sex")
            )?;
        cur_byte += 1;

        let num_genders = u32::from_le_bytes(get_bytes(data,cur_byte)?);
        cur_byte += 4;

        for _ in 0..num_genders {
            let mut g = Gender {
                ..Gender::default()
            };

            let gender_name_len = u32::from_le_bytes(get_bytes(data,cur_byte)?);
            cur_byte += 4;

            g.gender_name = Flexistring::Dynamic(String::from_utf8_lossy(get_bytes_arr(data,cur_byte,cur_byte+gender_name_len as usize)?).to_string());
            cur_byte += gender_name_len as usize;

            let subj_len = u8::from_le_bytes(get_bytes(data,cur_byte)?);
            cur_byte += 1;

            g.subj_pronoun = Flexistring::Dynamic(String::from_utf8_lossy(get_bytes_arr(data,cur_byte,cur_byte+subj_len as usize)?).to_string());
            cur_byte += subj_len as usize;

            let obj_len = u8::from_le_bytes(get_bytes(data,cur_byte)?);
            cur_byte += 1;

            g.obj_pronoun = Flexistring::Dynamic(String::from_utf8_lossy(get_bytes_arr(data,cur_byte,cur_byte+obj_len as usize)?).to_string());
            cur_byte += obj_len as usize;

            let pluralized = u8::from_le_bytes(get_bytes(data,cur_byte)?);
            cur_byte += 1;

            g.pluralized = if pluralized == 0x55 { true } 
                else if pluralized == 0xAA { false } 
                else {
                    return Result::Err(Box::new(io::Error::new(io::ErrorKind::InvalidData,"Invalid pluralization")));
                };

            ent.genders.push(g);
        }

        ent.position.0 = f32::from_le_bytes(get_bytes(data,cur_byte)?);
        cur_byte += 4;

        ent.position.1 = f32::from_le_bytes(get_bytes(data,cur_byte)?);
        cur_byte += 4;

        if cur_byte != old_cur + ent_size as usize {
            return Result::Err(Box::new(io::Error::new(io::ErrorKind::InvalidData,"Incorrect entity size")));
        }

        entities.push(ent);
    }


    Ok(entities)
}