use std::fs;

pub fn solve(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut safe_report_count = 0;
    let mut safe_report_count_with_dampening = 0;
    for line in contents.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if check_report(&nums) {
            safe_report_count += 1;
        }

        if check_with_dampening(&nums) {
            safe_report_count_with_dampening += 1;
        }
    }

    println!("number of safe reports: {safe_report_count}");
    println!("number of safe reports with dampening: {safe_report_count_with_dampening}");
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

fn copy_and_remove_elem(v: &Vec<i32>, x: usize) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();
    for i in 0..v.len() {
        if i != x {
            new_vec.push(v[i]);
        }
    }

    return new_vec;
}

fn check_with_dampening(v: &Vec<i32>) -> bool {
    if check_report(v) {
        return true;
    }

    for i in 0..v.len() {
        let dampened_report = copy_and_remove_elem(v, i);
        if check_report(&dampened_report) {
            return true;
        }
    }

   return false;
}
