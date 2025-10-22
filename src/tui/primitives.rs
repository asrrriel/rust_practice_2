use std::collections::HashMap;
use std::io::*;
use crate::base::gender::Gender;

pub fn input_string(prompt: &String) -> Result<String> {
    print!("{}",prompt);
    stdout().flush().unwrap();

    let mut str = String::new();
    match stdin().read_line(&mut str) {
        Ok(_) => return Result::Ok(str.trim().to_string()),
        Err(e) => return Err(e)
    };
}

pub fn input_int(prompt: &String) -> Result<i32> {
    let str = match input_string(&prompt) {
        Ok(s) => s,
        Err(e) => return Err(e)
    };

    return match str.parse() {
        Ok(v) => Result::Ok(v),
        Err(e) => Err(Error::new(ErrorKind::Other, e))
    }
}

pub fn input_one_of<T>(hm: &mut HashMap<String,T>,prompt: &String) -> T {
    loop {
        let str = match input_string(&prompt) {
            Ok(s) => s,
            Err(_) =>{
                println!("Failed to take your input!");
                continue;
            } 
        };

        match hm.contains_key(&str) {
            true => return hm.remove(&str).unwrap(),
            false => {
                println!("Input has to be one of:");
                for k in hm.keys() {
                    println!("   - \"{}\"",k);
                }
            }
        }
    }
}

pub fn input_gender<'a>() -> Result<Gender<'a>> {
    let gender_name = match input_string(&"Gender name[any string]: ".to_string()) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to take your input!");
            return Result::Err(e)
        }
    };

    let subj_pronoun = match input_string(&"Subject pronoun(e.g. he,she, it or they): ".to_string()) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to take your input!");
            return Result::Err(e)
        }
    };

    let obj_pronoun = match input_string(&"Object pronoun(e.g. him,her, it or them): ".to_string()) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to take your input!");
            return Result::Err(e)
        }
    };

    let mut hm = HashMap::<String,bool>::new();

    hm.insert("yes".to_string(),true);
    hm.insert("No".to_string(),false);

    let pluralized = input_one_of(&mut hm,&"Pluralizing(like themselves instead of themself)[yes/no]? ".to_string());

    Result::Ok(Gender {
            gender_name: gender_name.into(),
            subj_pronoun: subj_pronoun.into(),
            obj_pronoun: obj_pronoun.into(),
            pluralized: pluralized
    })
}
