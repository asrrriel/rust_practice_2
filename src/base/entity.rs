use super::gender::*;
use strum::IntoEnumIterator;
use strum_macros::{Display,EnumIter,FromRepr};

#[derive(Eq, Hash, PartialEq, Clone, Display, EnumIter, Default,FromRepr,Debug)]
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

impl Species {
    pub fn from_string(value: String) -> Option<Self> {
        Species::iter().find(|s|{ s.to_string().eq_ignore_ascii_case(&value)})
    }
}

#[derive(Default,Debug)]
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

        if let Some(v) = id{
            println!("ID:       {id}",
                id = v
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
