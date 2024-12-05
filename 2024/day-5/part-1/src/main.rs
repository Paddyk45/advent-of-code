use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let (ords, pgs) = file.split_once("\n\n").unwrap();

    let mut orders: Vec<(u8, u8)> = vec![];
    for line in ords.lines() {
        let (l, r) = line.split_once("|").unwrap();
        let l = l.parse::<u8>().unwrap();
        let r = r.parse::<u8>().unwrap();

        orders.push((l, r));
    }

    let mut pages: Vec<Vec<u8>> = vec![];
    for line in pgs.lines() {
        let ps = line
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u8>>();
        pages.push(ps);
    }

    let mut correct_pages = vec![];
    for ps in pages {
        let mut correct_order = true;
        for (i, n) in ps.iter().enumerate() {
            for rule in orders.iter().filter(|r| &r.1 == n) {
                if ps.iter().skip(i).any(|o| o == &rule.0) {
                    correct_order = false;
                    break;
                }
            }
        }

        if correct_order {
            correct_pages.push(ps);
        }
    }

    let mid_sum = correct_pages
        .iter()
        .fold(0u32, |acc, ps| acc + (ps[ps.len() / 2] as u32));

    println!("Sum: {mid_sum}");
}
