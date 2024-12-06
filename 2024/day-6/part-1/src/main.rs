use std::fs;

#[derive(PartialEq)]
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
    let mut visited_pos = vec![];
    visited_pos.push(pos);
    loop {
        let movm = rot.get_movement();
        let np0 = pos.0 + movm.0;
        let np1 = pos.1 + movm.1;
        if np0 < 0 || np0 > (width - 1) as i32 || np1 < 0 || np1 > (height - 1) as i32 {
            break;
        }
        if &map[np0 as usize][np1 as usize] == &MapChar::Obstacle {
            rot.rot_90_deg_r();
            continue;
        }

        pos.0 += rot.get_movement().0;
        pos.1 += rot.get_movement().1;
        visited_pos.push(pos);
    }
    
    visited_pos.sort();
    visited_pos.dedup();

    println!("visited {} tiles", visited_pos.len());
}
