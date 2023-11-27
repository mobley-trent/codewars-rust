use std::fs;

mod kata;

fn main() {
    let count = fs::read_dir(r"src\kata").unwrap().count();
    println!("The number of completed kata so far is {}", count - 1);
}
