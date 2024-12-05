use pcre2::bytes::{CaptureMatches, Captures, Regex};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let line_len = input.lines().nth(0).unwrap().len();

    let mut re_str = "(?=(XMAS|SAMX".to_string();

     // vertical - up-down
    for n in 0..line_len {
        let x = line_len - n - 1;
        re_str.push_str(&format!(
            "|(?<=X.{{{x}}}\\n.{{{n}}}M.{{{x}}}\\n.{{{n}}}A.{{{x}}}\\n.{{{n}}}S)"
        ));
    }

    // vertical - down-up
    for n in 0..line_len {
        let x = line_len - n - 1;
        re_str.push_str(&format!(
            "|(?>S.{{{x}}}\\n.{{{n}}}A.{{{x}}}\\n.{{{n}}}M.{{{x}}}\\n.{{{n}}}X)"
        ));
    }

    // diagonal - upleft-downright
    for n in 0..line_len-3 {
        re_str.push_str(&format!(
            "|(?>X.{{{x1}}}\\n.{{{n1}}}M.{{{x2}}}\\n.{{{n2}}}A.{{{x3}}}\\n.{{{n3}}}S)",
            x1 = line_len - n - 1,
            x2 = line_len - n - 2,
            x3 = line_len - n - 3,
            n1 = n + 1,
            n2 = n + 2,
            n3 = n + 3,
        ));
    }

    // diagonal - upright-downleft
    for n in 4..=line_len {
        re_str.push_str(&format!(
            "|(?>X.{{{x1}}}\\n.{{{n1}}}M.{{{x2}}}\\n.{{{n2}}}A.{{{x3}}}\\n.{{{n3}}}S)",
            x1 = line_len - n,
            x2 = line_len - n + 1,
            x3 = line_len - n + 2,
            n1 = n - 2,
            n2 = n - 3,
            n3 = n - 4,
        ));
    }


    // diagonal - downleft-upright
    for n in 0..line_len - 3 {
        re_str.push_str(&format!(
            "|(?>S.{{{x1}}}\\n.{{{n1}}}A.{{{x2}}}\\n.{{{n2}}}M.{{{x3}}}\\n.{{{n3}}}X)",
            x1 = line_len - n - 1,
            x2 = line_len - n - 2,
            x3 = line_len - n - 3,
            n1 = n + 1,
            n2 = n + 2,
            n3 = n + 3,
        ));
    }


    //  diagonal - downright-upleft
    for n in 4..=line_len {
        re_str.push_str(&format!(
            "|(?>S.{{{x1}}}\\n.{{{n1}}}A.{{{x2}}}\\n.{{{n2}}}M.{{{x3}}}\\n.{{{n3}}}X)",
            x1 = line_len - n,
            x2 = line_len - n + 1,
            x3 = line_len - n + 2,
            n1 = n - 2,
            n2 = n - 3,
            n3 = n - 4,
        ));
    }

    re_str.push_str("))");
    let re = Regex::new(&re_str).unwrap();
    println!("{re_str}");
    dbg!(re.captures_iter(input.as_bytes()).map(Result::unwrap).collect::<Vec<Captures>>().len());
}
