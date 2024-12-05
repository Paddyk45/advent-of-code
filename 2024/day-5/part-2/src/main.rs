use std::fs;

fn check(orders: &Vec<(u8, u8)>, pages: &Vec<u8>) -> Option<(usize, usize)> {
    for (i, n) in pages.iter().enumerate() {
        for rule in orders.iter().filter(|r| &r.1 == n) {
            if let Some(i2) = pages.iter().enumerate().skip(i).find_map(|(i, o)| {
                if o == &rule.0 {
                    Some(i)
                } else {
                    None
                }
            }) {
                return Some((i, i2))
            }
        }
    }
    
    None
}

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

    let mut corrected_pages: Vec<Vec<u8>> = vec![];
    dbg!(&pages);
    for mut ps in pages {
        //println!("{ps:?}");
        let mut is_correct = true;
        for (i, n) in ps.iter().enumerate() {
            for rule in orders.iter().filter(|r| &r.1 == n) {
                if ps.iter().skip(i).any(|o| o == &rule.0) {
                    is_correct = false;
                }
            }
        }

        if is_correct {
            println!("skipping {ps:?}");
            continue;
        }
        
        println!("doin {ps:?}");
        while let Some((p1, p2)) = check(&orders, &ps) {
            ps.swap(p1, p2)
        }
        dbg!(&ps);
        corrected_pages.push(ps);
        //panic!();
    }

    dbg!(&corrected_pages);

    let mid_sum = corrected_pages
        .iter()
        .fold(0u32, |acc, ps| acc + (ps[ps.len() / 2] as u32));

    println!("Sum: {mid_sum}");
}
