use super::gender::*;
use std::fmt::Display;

#[derive(Eq, Hash, PartialEq, Clone)]
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

impl Display for Species{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{0}", match self {
            Self::Cat      => "Cat",
            Self::Cattle   => "Cattle",
            Self::Dog      => "Dog",
            Self::Dragon   => "Dragon",
            Self::Elephant => "Elephant",
            Self::Fox      => "Fox",
            Self::Goat     => "Goat",
            Self::Human    => "Human",
            Self::Jay      => "Jay",
            Self::Lion     => "Lion",
            Self::Lizard   => "Lizard",
            Self::Ox       => "Ox",
            Self::Pig      => "Pig",
            Self::Shark    => "Shark",
            Self::Sheep    => "Sheep",
            Self::Sparrow  => "Sparrow",
            Self::Wolf     => "Wolf",
            Self::Zebra    => "Zebra",
        })
    }
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
