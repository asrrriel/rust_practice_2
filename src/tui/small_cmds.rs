use crate::base::animal::*;

pub fn cmd_list(animals: &Vec<Animal>) {
    for a in animals {
        a.describe();
    }
}