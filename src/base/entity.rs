use super::gender::*;
use strum::IntoEnumIterator;
use strum_macros::{Display,EnumIter,FromRepr};

#[derive(Eq, Hash, PartialEq, Clone, Display, EnumIter, Default,FromRepr)]
pub enum Species {
    Cat,
    Cattle,
    Dog,
    #[default] Dragon,
    Elephant,
    Fox,
    Goat,
    Human,
    Jay,
    Lion,
    Lizard,
    Ox,
    Pig,
    Shark,
    Sheep,
    Sparrow,
    Wolf,
    Zebra,
}

impl<'a> Species {
    pub fn from_string(value: String) -> Option<Self> {
        for s in Species::iter() {
            if s.to_string().to_ascii_lowercase() == value.to_ascii_lowercase(){
                return Some(s);
            }
        }

        return None;
    }
}

#[derive(Default)]
pub struct Entity<'a> {
    pub species: Species,
    pub age: u64,
    pub name: String,
    pub sex: Sex,
    pub genders: Vec<Gender<'a>>,
    pub position: (f32,f32)
}

impl Entity<'_> {
    pub fn describe(&self,id: Option<usize>) {
        println!("==={name}===",
            name = self.name,   
        );

        if id.is_some(){
            println!("ID:       {id}",
                id = id.unwrap()
            );
        }

        print!(r##"Species:  {species}
Age:      {age}
Sex:      {sex}
Gender(s):
{genders}
Position: ({pos_x},{pos_y})

"##,
            species = self.species,
            age = self.age,
            sex = self.sex,
            genders = display_genders(&self.genders),
            pos_x = self.position.0,
            pos_y = self.position.1
        );
    }
}
