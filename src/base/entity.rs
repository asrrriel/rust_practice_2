use super::gender::*;
use strum::IntoEnumIterator;
use strum_macros::{Display,EnumIter};

#[derive(Eq, Hash, PartialEq, Clone, Display, EnumIter)]
pub enum Species {
    Cat,
    Cattle,
    Dog,
    Dragon,
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

pub struct Enitiy<'a> {
    pub species: Species,
    pub age: u64,
    pub name: String,
    pub sex: Sex,
    pub genders: Vec<Gender<'a>>,
    pub position: (f32,f32)
}

impl Enitiy<'_> {
    pub fn describe(&self) {
        print!(r##"==={name}===
Species:  {species}
Age:      {age}
Sex:      {sex}
Gender(s):
{genders}
Position: ({pos_x},{pos_y})

"##,
            name = self.name,
            species = self.species,
            age = self.age,
            sex = self.sex,
            genders = display_genders(&self.genders),
            pos_x = self.position.0,
            pos_y = self.position.1
        );
    }
}
