use std::fs;

fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let file = fs::read_to_string("input.txt").expect("Failed to read file");
    let lines = file.lines().map(|l| l.strip_prefix("Game ").unwrap());
    let mut possible_games: Vec<i32> = vec![];
    for l in lines {
        // EXAMPLE LINE: `1: 3 blue, 3 red; 3 red, 3 green, 3 blue; 7 green`
        let gid_end = l.find(":").unwrap();
        let game_id = &l[..gid_end];
        let l = l.split_once(": ").unwrap().1;

        let parts = l.split("; ").map(|p| {
            let color_parts = p.split(", ");
            let c: Vec<(i32, &str)> = color_parts.map(|p| {
                let s = p.split_once(" ").unwrap();
                (s.0.parse().unwrap(), s.1)
            }).collect();
            c
        });

        let is_impossible = parts.map(|f| {
            let mut is_possible = true;
            for color in f {
                let possible = match color.1 {
                    "red" => color.0 <= max_red,
                    "green" => color.0 <= max_green,
                    "blue" => color.0 <= max_blue,
                    _ => unreachable!()
                };
                if !dbg!(possible) {
                    is_possible = false;
                }
            };
            is_possible
        }).any(|c| !c);

        if !is_impossible {
            possible_games.push(game_id.parse().unwrap());
        }
    }
    dbg!(possible_games.clone());
    let possible_games_sum: i32 = possible_games.into_iter().sum();
    dbg!(possible_games_sum);
}