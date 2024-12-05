use std::io;

pub mod bin {
    pub mod d1;
}
mod d2;
mod d4;
mod d5;

fn main() {
    println!("day to run:");
    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");

    match day.as_str().trim() {
        "d1"=>bin::d1::main(),
        "2" => d2::solve("./src/tests/data/d2/problem.txt"),
        // "3" => d3::solve("./src/tests/data/d3/problem.txt"),
        "4" => d4::solve("./src/tests/data/d4/problem.txt"),
        "5" => d5::solve("./src/tests/data/d5/problem.txt"),
        _ => println!("Error: day does not exist"),
    }
}
