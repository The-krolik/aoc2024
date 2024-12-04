use std::fs;

pub fn solve(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut safe_report_count = 0;
    for line in contents.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if check_report(&nums) {
            safe_report_count += 1;
        }
    }

    println!("number of safe reports: {safe_report_count}");
}

fn check_report2(r: &Vec<i32>) -> bool {
    let mut dampened = false;
    let mut i = 0;
    while i < v.len() {
         

        i += 1;
    }
}

fn check_report(r: &Vec<i32>) -> bool {
    if (check_for_increasing(r) | check_for_decreasing(r)) & check_for_gradual_slope(r) {
        return true;
    } else {
        return false;
    }
}

fn check_for_increasing(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if v[i] <= v[i-1] {
            return false;
        }
    }

    return true;
}

fn check_for_decreasing(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if v[i] >= v[i-1] {
            return false;
        }
    }

    return true;
}

fn check_for_gradual_slope(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        let diff = i32::abs(v[i] - v[i-1]);
        if (diff < 1) | (diff > 3) {
            return false;
        }
    }

    return true;
}
