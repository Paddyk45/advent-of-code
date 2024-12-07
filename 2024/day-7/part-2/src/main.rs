use std::fs;

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn apply(&self, n1: u64, n2: u64) -> u64 {
        match self {
            Self::Add => n1 + n2,
            Self::Mul => n1 * n2,
            Self::Concat => format!("{n1}{n2}").parse().unwrap(),
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut equations = vec![];
    for line in file.lines() {
        let (res, nums) = line.split_once(": ").unwrap();
        let nums = nums.split(" ").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        equations.push((res.parse::<u64>().unwrap(), nums));
    }

    let mut found_sum = 0;
    println!("This code will take a long time to finish because i suck at optimizing, sorry!");
    for eq in equations {
        let nums = eq.1;
        'lo: for bits in 0u32..=u32::MAX >> (32 - (nums.len()-1) * 2) {
            let mut operators = vec![];
            for n in 0..nums.len() - 1 {
                let op = bits & 3u32 << n * 2;
                if op == 0 {
                    operators.push(Operator::Add);
                } else if op >> n * 2 == 1 {
                    operators.push(Operator::Mul);
                } else if op >> n * 2 == 2 {
                    continue 'lo;
                } else {
                    operators.push(Operator::Concat);
                }
            }
            let mut sum = nums[0];
            for (i, n) in nums.iter().skip(1).enumerate() {
                sum = operators[i].apply(sum, *n);
            }

            if sum == eq.0 {
                found_sum += sum;
                break 'lo;
            }
        }
    }

    println!("Sum: {found_sum}");
}
