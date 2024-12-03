use regex::Regex;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read line");

    let re = Regex::new("(mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\)|)").unwrap();
    let (_, sum) = re
        .captures_iter(&file)
        .fold((true, 0), |(mut en, mut acc), cap| {
            if &cap[0] == "do()" {
                (true, acc)
            } else if &cap[0] == "don't()" {
                (false, acc)
            } else if en {
                // idk, seems to work
                if &cap[0] == "" {
                    return (en, acc);
                }

                (
                    en,
                    acc + cap[2].parse::<i32>().unwrap() * cap[3].parse::<i32>().unwrap(),
                )
            } else {
                unreachable!()
            }
        });
    println!("Sum: {sum}");
}
