use std::fs;
use std::io;

pub fn main() {
    println!("filename:");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");

    let (v1, v2) = get_numbers_from_file("./src/tests/data/d1/p.txt");

    let total_distance = calculate_total_distance(&v1, &v2);
    let similarity_score = calculate_similarity_score(&v1, &v2);

    println!("total distance: {total_distance}");
    println!("similarity score: {similarity_score}");
}

fn calculate_total_distance(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut total_distance = 0;
    for i in 0..v1.len() {
        total_distance += i32::abs(v1[i] - v2[i]);
    }

    return total_distance;
}

fn calculate_similarity_score(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut score = 0;
    for n in v1.iter() {
        let occurences = v2.iter().filter(|&x| *x == *n).count();
        score += *n * (occurences as i32);
    }

    return score;
}

fn get_numbers_from_file(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let nums = line.split_whitespace().collect::<Vec<&str>>();
        let num1 = nums[0].parse().expect("number to be parsable to i32");
        let num2 = nums[1].parse().expect("number to be parsable to i32");
        v1.push(num1);
        v2.push(num2);
    }

    v1.sort();
    v2.sort();

    return (v1, v2);
}
