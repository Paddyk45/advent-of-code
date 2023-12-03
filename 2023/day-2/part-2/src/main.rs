use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read file");
    let lines = file.lines().map(|l| l.strip_prefix("Game ").unwrap());
    let mut powers: Vec<i32> = vec![];
    for l in lines {
        // EXAMPLE LINE: `1: 3 blue, 3 red; 3 red, 3 green, 3 blue; 7 green`
        let l = l.split_once(": ").unwrap().1;

        let parts = l.split("; ").map(|p| {
            let color_parts = p.split(", ");
            let c: Vec<(i32, &str)> = color_parts
                .map(|p| {
                    let s = p.split_once(" ").unwrap();
                    (s.0.parse().unwrap(), s.1)
                })
                .collect();
            c
        });

        let (r_min, g_min, b_min) = {
            let mut rgbm = (0, 0, 0);
            parts.for_each(|c| {
                c.iter().for_each(|col| match col.1 {
                    "red" => {
                        if col.0 > rgbm.0 {
                            rgbm.0 = col.0
                        }
                    }
                    "green" => {
                        if col.0 > rgbm.1 {
                            rgbm.1 = col.0
                        }
                    }
                    "blue" => {
                        if col.0 > rgbm.2 {
                            rgbm.2 = col.0
                        }
                    }
                    _ => unreachable!(),
                })
            });
            rgbm
        };
        powers.push(r_min * g_min * b_min);
    }
    let sum: i32 = powers.iter().sum();
    dbg!(sum);
}
