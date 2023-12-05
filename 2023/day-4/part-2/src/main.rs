use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_nums: usize,
}

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
    let mut all_cards = vec![];
    for (i, (win_nums, your_nums)) in cards.into_iter().enumerate() {
        let hm_w: HashSet<u8> = HashSet::from_iter(win_nums);
        let hm_y: HashSet<u8> = HashSet::from_iter(your_nums);
        let matching_nums = hm_w.intersection(&hm_y).count();
        all_cards.push(
            Card {
                id: i,
                winning_nums: matching_nums
            }
        )
    }
    let mut cards_amount = vec![0; all_cards.len()];
    let mut sum = 0;

    for card in all_cards.iter() {
        let start = card.id + 1;
        let end = card.id + card.winning_nums;
        for i in start..=end {
            cards_amount[i] += 1 + cards_amount[card.id]
        }
        sum += 1 + cards_amount[card.id];
    }
    dbg!(sum);
}
