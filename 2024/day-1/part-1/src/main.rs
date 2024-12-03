use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read line");
    let mut l1: Vec<i32> = vec![];
    let mut l2: Vec<i32> = vec![];
    for l in file.lines() {
        let (l, r) = l.split_once("   ").unwrap();
        l1.push(l.parse().unwrap());
        l2.push(r.parse().unwrap());
    }

    l1.sort();
    l2.sort();

    let sum = l1
        .iter()
        .zip(l2)
        .fold(0i32, |acc, (x, y)| acc + (y - x).abs());

    println!("Result: {sum}")
}
