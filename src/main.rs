use std::fs;

// 7 kyu problems:
mod cc_mask;
mod find_the_divisors;

fn main() {
    let count = fs::read_dir("src").unwrap().count();
    println!("The number of completed kata so far is {}", count - 1);
}
