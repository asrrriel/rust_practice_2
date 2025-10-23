use std::collections::BTreeMap;
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
    let str = input_string(&prompt)?;

    return match str.parse() {
        Ok(v) => Result::Ok(v),
        Err(e) => Err(Error::new(ErrorKind::Other, e))
    }
}

pub fn input_one_of<T>(hm: &mut BTreeMap<String,T>,prompt: &String) -> Result<T> {
    loop {
        let str = input_string(&prompt)?;

        match hm.contains_key(&str) {
            true => return Ok(hm.remove(&str).unwrap()),
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
    let gender_name = input_string(&"Gender name[any string]: ".to_string())?;

    let subj_pronoun = input_string(&"Subject pronoun(e.g. he,she, it or they): ".to_string())?;

    let obj_pronoun = input_string(&"Object pronoun(e.g. him,her, it or them): ".to_string())?;

    let mut m = BTreeMap::<String,bool>::new();

    m.insert("yes".to_string(),true);
    m.insert("No".to_string(),false);

    let pluralized = input_one_of(&mut m,&"Pluralizing(like themselves instead of themself)[yes/no]? ".to_string())?;

    Result::Ok(Gender {
            gender_name: gender_name.into(),
            subj_pronoun: subj_pronoun.into(),
            obj_pronoun: obj_pronoun.into(),
            pluralized: pluralized
    })
}
