use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read line");

    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
    let sum = re.captures_iter(&file).fold(0, |acc, cap| {
        acc + cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap()
    });
    println!("Sum: {sum}");
}
