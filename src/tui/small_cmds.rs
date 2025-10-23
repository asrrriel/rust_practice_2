use crate::base::entity::*;

pub fn cmd_list(entities: &Vec<Enitiy>) {
    for a in entities {
        a.describe();
    }
}

pub fn cmd_clear() {
    print!("\x1B[2J\x1B[1;1H"); //ANSI clear and cursore move to 1;1
}