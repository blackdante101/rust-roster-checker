use std::fs;
use std::env;
fn main() {
    let name_input = env::args().nth(1).unwrap();
    let file_contents = fs::read_to_string("roster.txt").unwrap();

    for line in file_contents.lines() {
        if name_input == line {
            println!("{} is part of the roster", name_input);
            return;
        }
    }

    println!("{} is not part of the roster", name_input); 
}
