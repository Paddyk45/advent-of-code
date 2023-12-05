use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Failed to read file");
    // strip "Card X: "
    let lines = file
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.to_string());
    // split winning numbers and your numbers
    let wn_yn = lines.map(|l| {
        let split = l.split_once(" | ").unwrap();
        (split.0.to_string(), split.1.to_string())
    });
    // parse string into vec of numbers
    let cards: Vec<(Vec<u8>, Vec<u8>)> = wn_yn
        .map(|(w, y)| {
            let winning_nums: Vec<u8> = w
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let your_nums: Vec<u8> = y
                .split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (winning_nums, your_nums)
        })
        .collect();
    let mut points = 0u32;
    for (win_nums, your_nums) in cards {
        let hm_w: HashSet<u8> = HashSet::from_iter(win_nums);
        let hm_y: HashSet<u8> = HashSet::from_iter(your_nums);
        let mut pts = 0.5;
        for _ in 0..hm_w.intersection(&hm_y).count() {
            pts *= 2.0;
        }
        points += pts as u32;
    }
    dbg!(points);
}
