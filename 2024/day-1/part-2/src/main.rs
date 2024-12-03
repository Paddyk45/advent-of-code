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

    let l2i = l2.into_iter();

    let diff_score = l1.iter().fold(0i32, |acc, x| {
        acc + (l2i.clone().filter(|y| *x == *y).count() as i32 * x)
    });

    println!("Diff score: {diff_score}");
}
