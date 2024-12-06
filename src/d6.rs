use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(file_path: &str) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut G: HashMap<(i32, i32), char> = HashMap::new();
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            let a = i as i32;
            let b = j as i32;
            G.insert((a, b), c);
        }
    }

    let mut start: (i32, i32) = (0, 0);
    for (key, val) in G.iter() {
        if *val == '^' {
            start = *key;
        }
    }

    let path = walk(&G, start).0;
    let mut sum = 0;
    for pos in path.iter() {
        if pos.0 == start.0 && pos.1 == start.1 {
            continue;
        }

        let temp = *G.get(pos).unwrap();
        G.insert(*pos, '#');
        if walk(&G, start).1 {
            sum += 1;
        }
        G.insert(*pos, temp);
    }

    println!("{} : {}", path.len(), sum);
}

fn walk(G: &HashMap<(i32, i32), char>, start: (i32, i32)) -> (HashSet<(i32, i32)>, bool) {
    let mut pos = start;
    let mut dir: (i32, i32) = (-1, 0);
    let mut seen = HashSet::new();

    while G.contains_key(&pos) && !seen.contains(&(pos, dir)) {
        seen.insert((pos, dir));
        let sum = (pos.0 + dir.0, pos.1 + dir.1);
        if G.contains_key(&sum) && *G.get(&sum).unwrap() == '#' {
            dir = (dir.1, dir.0 * -1);
        }
        else {
            pos = sum;
        }
    }

    return (seen.iter().map(|x| x.0).collect(), seen.contains(&(pos, dir)));
}
