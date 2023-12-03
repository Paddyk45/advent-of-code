use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Number {
    line: usize,
    start: usize,
    end: usize,
    value: u32,
}

#[derive(Debug)]
struct Gear {
    line: usize,
    pos: usize,
}

fn try_get_number(line: usize, column: usize, numbers: &Vec<Number>) -> Option<&Number> {
    for n in numbers.into_iter().filter(|n| n.line == line) {
        if n.start <= column && n.end >= column {
            return Some(n);
        }
    }
    None
}

impl Gear {
    pub fn get_adj_numbers<'a>(&self, numbers: &Vec<Number>) -> Vec<Number> {
        //let chars: Vec<Vec<char>> = get_file().lines().map(|l| l.chars().collect()).collect();
        let mut nums: Vec<Number> = vec![];
        for l in self.line - 1..=self.line + 1 {
            for c in self.pos..self.pos + 2 {
                if let Some(n) = try_get_number(l, c, numbers) {
                    nums.push(n.clone());
                }
            }
        }

        nums.dedup_by(|x, y| x == y);
        nums
    }
}

fn get_file() -> String {
    fs::read_to_string("input.txt").expect("Failed to read file")
}

fn main() {
    let file = get_file();
    let lines: Vec<&str> = file.lines().collect();

    let mut numbers: Vec<Number> = vec![];
    let mut gears: Vec<Gear> = vec![];

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
            if c == '*' {
                gears.push(Gear { line: l_idx, pos })
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
    //dbg!( gears.iter().map(|g| g.get_adj_numbers(&numbers)).collect::<Vec<Vec<Number>>>() );
    let ratios = gears.iter().filter_map(|g| -> Option<u32> {
        let adj_nums = g.get_adj_numbers(&numbers);
        if adj_nums.len() == 2 {
            return Some(adj_nums[0].value * adj_nums[1].value);
        }
        None
    });

    let sum = ratios.fold(0, |sum, r| sum + r);
    dbg!(sum);
}
