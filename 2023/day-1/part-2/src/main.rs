use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read line");
    let lines_stripped = file.lines().map(|l| {
        let rl = l.replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        let mut s = String::new();
        rl.chars().filter(char::is_ascii_digit).for_each(|c| s.push(c));
        s
    });
    let mut sum = 0;
    for ln in lines_stripped {
        if ln.len() == 1 {
            sum += ln.parse::<i32>().unwrap() * 11;
            continue;
        }
        let c1 = ln.chars().next().unwrap();
        let c2 = ln.chars().last().unwrap();
        sum += format!("{c1}{c2}").parse::<i32>().unwrap();
    }
    dbg!(sum);
}
