use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input");

    let field: HashMap<_, _> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c != '.').then_some(((x as i64, y as i64), c)))
        })
        .flatten()
        .collect();

    let (&(x, y), _) = field.iter().find(|(_, &c)| c == 'S').unwrap();

    let mut startings_points: [(i64, i64); 2] = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|pos| {
            let Some(c) = field.get(pos) else {
                return false;
            };
            match (pos.0 - x, pos.1 - y) {
                (-1, 0) => ['-', 'L', 'F'].contains(c),
                (1, 0) => ['-', 'J', '7'].contains(c),
                (0, -1) => ['|', 'F', '7'].contains(c),
                (0, 1) => ['|', 'L', 'J'].contains(c),
                unhandled => todo!("{unhandled:?}"),
            }
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut old = (x, y);
    let mut current = startings_points[0];

    let mut path: HashSet<_> = [old, current].into();
    loop {
        let next = match field[&current] {
            '|' => {
                if old.1 < current.1 {
                    (current.0, current.1 + 1)
                } else {
                    (current.0, current.1 - 1)
                }
            }
            'J' => {
                if old.1 == current.1 {
                    (current.0, current.1 - 1)
                } else {
                    (current.0 - 1, current.1)
                }
            }
            'F' => {
                if old.1 == current.1 {
                    (current.0, current.1 + 1)
                } else {
                    (current.0 + 1, current.1)
                }
            }
            '7' => {
                if old.1 == current.1 {
                    (current.0, current.1 + 1)
                } else {
                    (current.0 - 1, current.1)
                }
            }
            'L' => {
                if old.1 == current.1 {
                    (current.0, current.1 - 1)
                } else {
                    (current.0 + 1, current.1)
                }
            }
            '-' => {
                if old.0 < current.0 {
                    (current.0 + 1, current.1)
                } else {
                    (current.0 - 1, current.1)
                }
            }
            'S' => break,
            unexpected => todo!("{unexpected}"),
        };

        old = current;
        current = next;
        // steps += 1;
        path.insert(current);
    }

    let max_x = path.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = path.iter().map(|(_, y)| *y).max().unwrap();

    let mut inside_count = 0;
    for y in 0..=max_y {
        let mut inside = false;
        let mut previous_was = false;
        for x in 0..=max_x {
            if path.contains(&(x, y)) {
                if !['-', 'L', 'J'].contains(&field[&(x, y)]) {
                    inside = !inside;
                }
            } else {
                if inside {
                    println!("{x} {y}");
                    inside_count += 1;
                }
            }
        }
    }

    println!("{inside_count}");

    // println!("{}", steps / 2);
}
