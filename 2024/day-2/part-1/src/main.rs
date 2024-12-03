use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read line");
    let mut levels: Vec<Vec<i32>> = vec![];

    for l in file.lines() {
        let le = l.split(" ").map(|s| s.parse().unwrap()).collect();
        levels.push(le);
    }

    let mut n_safe = 0;
    for l in levels {
        let asc = l[0] < l[1];
        let is_unsafe = l
            .iter()
            .zip(l.iter().skip(1))
            .any(|(x, y)| !is_safe(asc, x, y));
        if !is_unsafe {
            n_safe += 1;
        }
    }

    println!("N Safe: {n_safe}")
}

fn is_safe(asc: bool, x: &i32, y: &i32) -> bool {
    (if asc { x < y } else { x > y }) && (x - y).abs() <= 3
}
