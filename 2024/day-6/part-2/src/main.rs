use std::fs;

#[derive(PartialEq, Clone)]
enum MapChar {
    Empty,
    Obstacle,
}

impl MapChar {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Obstacle,
            '.' | '^' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

impl Rotation {
    fn rot_90_deg_r(&mut self) {
        let r = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        };

        *self = r;
    }

    fn get_movement(&self) -> (i32, i32) {
        match self {
            Self::Up => (-1, 0),
            Self::Right => (0, 1),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut map = vec![];
    let mut pos = (0i32, 0i32);
    let mut rot = Rotation::Up;
    for (i, l) in file.lines().enumerate() {
        let chars = l
            .chars()
            .enumerate()
            .map(|(j, c)| {
                if c == '^' {
                    pos = (i as i32, j as i32);
                    return MapChar::Empty;
                }
                MapChar::from_char(c)
            })
            .collect::<Vec<MapChar>>();
        map.push(chars);
    }

    let width = map[0].len();
    let height = map.len();
    let mut n_loops = 0;

    for x in 0..height {
        for y in 0..width {
            let mut map = map.clone();
            let mut player_pos = pos.clone();
            let mut rot = rot.clone();
            if map[x][y] == MapChar::Obstacle {
                continue;
            }

            map[x][y] = MapChar::Obstacle;
            let mut did_finish = false;

            for _ in 0..100000 { // break after at most a gazillion steps (mm yes very fast)
                let movm = rot.get_movement();
                let np0 = player_pos.0 + movm.0;
                let np1 = player_pos.1 + movm.1;
                if np0 < 0 || np0 > (width - 1) as i32 || np1 < 0 || np1 > (height - 1) as i32 {
                    did_finish = true;
                    break;
                }
                if &map[np0 as usize][np1 as usize] == &MapChar::Obstacle {
                    rot.rot_90_deg_r();
                    continue;
                }

                player_pos.0 += rot.get_movement().0;
                player_pos.1 += rot.get_movement().1;
            }

            if !did_finish {
                n_loops += 1;
            }
        }
    }

    println!("Found {n_loops} loops");
}
