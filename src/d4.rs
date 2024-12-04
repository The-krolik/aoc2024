use std::fs;

pub fn solve(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let chars = contents.as_str().chars();

    let mut word_search: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    for c in chars {
        match c {
            '\n' => {
                word_search.push(row);
                row = Vec::new();
            },
            _ => row.push(c),
        }
    }

    let word = vec!['X', 'M', 'A', 'S'];
    let search_count = search(&word_search, &word);

    let letters = vec!['M', 'S'];
    let xsearch_count = xsearch(&word_search, 'A', &letters);

    println!("{search_count} - {xsearch_count}");
}

fn search(word_search: &Vec<Vec<char>>, word: &Vec<char>) -> i32 {
    let x_directions: [i32; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];
    let y_directions: [i32; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];

    let mut count = 0;
    for y in 0..word_search.len() {
        for x in 0..word_search[y].len() {
            for i in 0..x_directions.len() {
                if check_letter(&word_search, x, y, &word, 0, x_directions[i], y_directions[i]) {
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn check_letter(word_search: &Vec<Vec<char>>, x: usize, y: usize, word: &Vec<char>, word_index: usize, xdir: i32, ydir: i32) -> bool {
    if word_index == word.len() {
        return true;
    }
    if y < 0 || y >= word_search.len() {
        return false;
    }
    if x < 0 || x >= word_search[y].len() {
        return false;
    }

    if word_search[y][x] == word[word_index] {
        let new_x: usize = (x as i32 + xdir) as usize;
        let new_y: usize = (y as i32 + ydir) as usize;
        return check_letter(&word_search, new_x, new_y, &word, word_index + 1, xdir, ydir);
    }

    return false;
}

fn xsearch(word_search: &Vec<Vec<char>>, pivot: char, letters: &Vec<char>) -> i32 {
    let x_directions: [i32; 4] = [0, 1, 0, -1];
    let y_directions: [i32; 4] = [-1, 0, 1, 0];

    let mut count = 0;
    for y in 1..word_search.len() - 1 {
        for x in 1..word_search[y].len() - 1 {
            if word_search[y][x] == pivot && check_cross(&word_search, x, y, &letters) {
                count += 1;
            }
        }
    }

    return count;
}

fn check_cross(word_search: &Vec<Vec<char>>, x: usize, y: usize, letters: &Vec<char>) -> bool {
    let upper_left = word_search[y - 1][x - 1];
    let lower_left = word_search[y + 1][x - 1];
    let upper_right = word_search[y - 1][x + 1];
    let lower_right = word_search[y + 1][x + 1];

    if !(letters.contains(&upper_left)
        && letters.contains(&upper_right)
        && letters.contains(&lower_left)
        && letters.contains(&lower_right)) {
           return false;
    }
    if upper_left == lower_left
        && upper_right == lower_right
        && upper_left != upper_right {
            return true;
    }
    if upper_left == upper_right
        && lower_left == lower_right
        && upper_left != lower_left {
            return true;
    }

    return false;
}
