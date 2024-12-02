use std::io;

pub mod bin {
    pub mod d1;
}

fn main() {
    println!("day to run:");
    let mut day = String::new();
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");

    match day.as_str().trim() {
        "d1"=>bin::d1::main(),
        _ => println!("Error: day does not exist"),
    }
}
