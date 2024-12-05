use std::fs;
use std::collections::HashMap;


pub fn solve(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let parts: Vec<&str> = contents.split("\n\n").collect();

    let mut page_rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in parts[0].lines() {
        let pages: Vec<i32> = rule
            .split("|")
            .map(|x| x.parse().unwrap())
            .collect();

        if page_rules.contains_key(&pages[0]) {
            let following_pages = page_rules.get_mut(&pages[0]).unwrap();
            following_pages.push(pages[1]);
        }
        else {
            let following_pages = vec![pages[1]];
            page_rules.insert(pages[0], following_pages);
        }
    }

    let mut updates: Vec<Vec<i32>> = parts[1]
        .lines()
        .map(|line| line.replace("\n", "").split(",").map(|x| x.parse().unwrap()).collect())
        .collect();

    let mut sum_of_middles: i32 = 0;
    let mut sum_of_fixed_middles: i32 = 0;
    for mut update in updates.iter_mut() {
        if check_update(&update, &page_rules) {
            sum_of_middles += update[update.len() / 2];
        }
        else {
            let end = update.len() - 1;
            merge_sort(&page_rules, &mut update, 0, end);
            sum_of_fixed_middles += update[update.len() / 2];
        }
    }

    println!("sum of middles from valid updates: {sum_of_middles}");
    println!("sum of middles from fixed updates: {sum_of_fixed_middles}");
}

fn check_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..update.len() {
        if rules.contains_key(&update[i]) {
            if !validate_order(update, i, rules.get(&update[i]).unwrap()) {
                return false;
            }
        }
    }

    return true;
}

fn validate_order(update: &Vec<i32>, last_index: usize, rules_for_page: &Vec<i32>) -> bool {
    for i in 0..last_index {
        for rule in rules_for_page.iter() {
            if update[i] == *rule {
                return false;
            }
        }
    }

    return true;
}

fn compare(page1: i32, page2: i32, rules: &HashMap<i32, Vec<i32>>) -> bool {
    if rules.contains_key(&page1) && rules.get(&page1).unwrap().contains(&page2) {
        return true;
    }

    if rules.contains_key(&page2) && rules.get(&page2).unwrap().contains(&page1) {
        return false;
    }
    
    return true;
}

fn merge_sort(rules: &HashMap<i32, Vec<i32>>, update: &mut Vec<i32>, start: usize, end: usize) {
    if end - start == 0 {
        return;
    }
    if end - start == 1 {
        if compare(update[start], update[end], &rules) {
            return;
        }
        else {
            update.swap(start, end);
            return;
        }
    }

    let middle = start + ((end - start) / 2);
    merge_sort(rules, update, start, middle);
    merge_sort(rules, update, middle + 1, end);
    merge(rules, update, start, end);
}

fn merge(rules: &HashMap<i32, Vec<i32>>, update: &mut Vec<i32>, start: usize, end: usize) {
    let mut middle = start + (end - start) / 2;
    let (mut i, mut j) = (start, middle + 1);

    while i <= middle && j <= end {
        if compare(update[i], update[j], rules) {
            i += 1;
        }
        else {
            move_elem(update, j, i);
            i += 1;
            j += 1;
            middle += 1;
        }
    }
}

fn move_elem(arr: &mut Vec<i32>, old_index: usize, new_index: usize) {
    if old_index < new_index {
        arr[old_index..=new_index].rotate_left(1);
    } else {
        arr[new_index..=old_index].rotate_right(1);
    }
}
