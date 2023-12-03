use std::fs;

#[derive(Debug)]
struct Number {
    line: usize,
    start: usize,
    end: usize,
    value: u32,
}

fn get_file() -> String {
    fs::read_to_string("input.txt").expect("Failed to read file")
}

fn is_symbol(c: &char) -> bool {
    !c.is_ascii_digit() && *c != '.'
}

impl Number {
    pub fn is_adj_to_sym(&self) -> bool {
        let line_chars: Vec<Vec<char>> = get_file()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let line_len = line_chars[0].len();

        let start = self.start - 1;
        let end = self.end + 1;

        // SYMBOLS ABOVE NUMBER
        if self.line != 0 {
            let mut chars = line_chars[self.line - 1][start..end].iter();
            if chars.any(is_symbol) {
                return true;
            }
        };

        // SYMBOL LEFT TO NUMBER
        if self.start != 0 {
            if is_symbol(&line_chars[self.line][start]) {
                return true;
            }
        }

        // SYMBOL RIGHT TO NUMBER
        if self.end != line_len - 1 {
            if is_symbol(&line_chars[self.line][self.end]) {
                return true;
            }
        }

        // SYMBOLS UNDER NUMBER
        if self.line + 1 < line_chars.len() {
            let mut chars = line_chars[self.line + 1][start..end].iter();
            if chars.any(is_symbol) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let file = get_file();
    let lines: Vec<&str> = file.lines().collect();

    let mut numbers: Vec<Number> = vec![];

    for (l_idx, l) in lines.iter().enumerate() {
        let mut number_tmp = (String::new(), 0);
        for (pos, c) in l.chars().enumerate() {
            if c.is_ascii_digit() {
                number_tmp.0.push(c);
                if number_tmp.1 == 0 {
                    number_tmp.1 = pos;
                }
            } else if !number_tmp.0.is_empty() {
                numbers.push(Number {
                    line: l_idx,
                    start: number_tmp.1,
                    end: pos,
                    value: number_tmp.0.parse().unwrap(),
                });
                number_tmp = (String::new(), 0);
            }
        }
        if number_tmp.1 != 0 {
            numbers.push(Number {
                line: l_idx,
                start: number_tmp.1,
                end: l.len() - 1,
                value: number_tmp.0.parse().unwrap(),
            })
        }
    }
    //dbg!(&numbers);
    let schem_nums = numbers.iter().filter(|n| n.is_adj_to_sym());
    let sum = schem_nums.fold(0, |s, n| s + n.value);
    dbg!(sum);
}
