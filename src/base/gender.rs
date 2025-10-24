use crate::util::flexistring::Flexistring;
use strum::IntoEnumIterator;
use strum_macros::{Display,EnumIter,FromRepr};

#[derive(PartialEq, Eq,Display,EnumIter,Clone,Default,FromRepr,Debug)]
pub enum Sex {
    Male,
    Female,
    #[default] Intersex
}

impl Sex {
    pub fn from_string(value: String) -> Option<Self> {
       Sex::iter().find(|v| v.to_string().eq_ignore_ascii_case(&value))
    }
}

#[derive(Clone, Default, Debug)]
pub struct Gender<'a> {
    pub gender_name: Flexistring<'a>,

    //grammatical aspect
    pub subj_pronoun: Flexistring<'a>,
    pub obj_pronoun: Flexistring<'a>,
    pub pluralized: bool //grammatical thingy, themselves is still themselves for singular they
}

pub const G_MALE: Gender = Gender{
    gender_name: Flexistring::Static("Male"),
    subj_pronoun: Flexistring::Static("he"),
    obj_pronoun: Flexistring::Static("him"),
    pluralized: false
};

pub const G_FEMALE: Gender = Gender{
    gender_name: Flexistring::Static("Female"),
    subj_pronoun: Flexistring::Static("she"),
    obj_pronoun: Flexistring::Static("her"),
    pluralized: false
};

pub const G_NON_BINARY: Gender = Gender{
    gender_name: Flexistring::Static("Non-binary"),
    subj_pronoun: Flexistring::Static("they"),
    obj_pronoun: Flexistring::Static("them"),
    pluralized: true
};

pub enum PronounType {
    Subject,
    Object,
    Reflexive
}

impl<'a> Gender<'a> {
    pub fn construct_pronoun(&self, t: PronounType) -> String{
        match t {
            PronounType::Subject    => self.subj_pronoun.to_string(),
            PronounType::Object     => self.obj_pronoun.to_string(),
            PronounType::Reflexive  => format!("{0}{1}",self.obj_pronoun,
                                                    if self.pluralized {"sleves"} else { "self" })
        }
    }
}

pub fn display_genders<'a>(genders: &Vec<Gender<'a>>) -> String{
    let mut str = String::new();

    for g in genders{
        str += format!("          {name}: {sub}/{obj}/{rfx}\n",
            name = g.gender_name,
            sub  = g.construct_pronoun(PronounType::Subject),
            obj  = g.construct_pronoun(PronounType::Object),
            rfx  = g.construct_pronoun(PronounType::Reflexive),

        ).as_str();
    }

    str
}