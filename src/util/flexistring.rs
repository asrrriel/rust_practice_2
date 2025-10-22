use std::fmt::Display;

#[derive(Clone)]
pub enum Flexistring<'a> {
    Static(&'a str),
    Dynamic(String)
}

impl<'a> Flexistring<'a> {
    pub fn to_string(&self) -> String{
        match self {
            Flexistring::Static(s) => s.to_string(),
            Flexistring::Dynamic(s) => s.to_string()
        }
    }
}

impl<'a> From<&'a str> for Flexistring<'a> {
    fn from(value: &'a str) -> Self {
        Flexistring::Static(value)
    }
}

impl From<String> for Flexistring<'_> {
    fn from(value: String) -> Self {
        Flexistring::Dynamic(value)
    }
}

impl Display for Flexistring<'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.to_string())
    }
}