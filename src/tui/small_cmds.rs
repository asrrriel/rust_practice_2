use crate::base::animal::*;

pub fn cmd_list(animals: &Vec<Animal>) {
    for a in animals {
        a.describe();
    }
}

pub fn cmd_clear() {
    print!("\x1B[2J\x1B[1;1H"); //ANSI clear and cursore move to 1;1
}