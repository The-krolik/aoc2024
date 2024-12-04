use std::fs;
use regex::Regex;

pub fn solve(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .replace("\n", "");
    let corrupted_memory = contents.as_str();

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let valid_expressions: Vec<&str> = re.find_iter(corrupted_memory).map(|e| e.as_str()).collect();

    let mut sum = 0;
    let mut doing = true;
    for exp in valid_expressions.iter() {
        match *exp {
            "do()" => doing = true,
            "don't()" => doing = false,
            _ => if doing { sum += calculate_expression(exp); }
        }
    }

    println!("{sum}");
}

fn calculate_expression(exp: &str) -> i32 {
    let re = Regex::new(r"[0-9]{1,3}").unwrap();
    let nums: Vec<i32> = re
        .find_iter(exp)
        .map(|n| n.as_str().parse().unwrap())
        .collect();

    return nums[0] * nums[1];
}
