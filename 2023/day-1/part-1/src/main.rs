use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read line");
    let lines_stripped = file.lines().map(|l| {
        let mut f = String::new();
        for c in l.chars() {
            if c.is_ascii_digit() {
                f.push(c);
            }
        }
        f
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
