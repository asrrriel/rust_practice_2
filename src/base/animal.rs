use super::gender::*;
use strum_macros::Display;

#[derive(Eq, Hash, PartialEq, Clone, Display)]
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

pub struct Animal<'a> {
    pub species: Species,
    pub age: u64,
    pub name: String,
    pub sex: Sex,
    pub genders: Vec<Gender<'a>>,
    pub position: (f32,f32)
}

impl Animal<'_> {
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
