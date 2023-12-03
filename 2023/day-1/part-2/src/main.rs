use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read line");
    let lines_stripped = file.lines().map(|l| {
        let mut s = String::new();
        for c in l.chars() {
            s.push(c);
            let repl_s = s
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9");
            if repl_s != s {
                s = repl_s;
                break;
            }
        }
        let mut sr = String::new();
        for c in l.chars().rev() {
            sr = format!("{c}{sr}");
            let repl_sr = sr
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9");
            if repl_sr != sr {
                sr = repl_sr;
                break;
            }
        }
        s.push_str(&sr);
        let mut f = String::new();
        for c in s.chars() {
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
