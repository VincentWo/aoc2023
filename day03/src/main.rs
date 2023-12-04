use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let mut symbols = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c.is_digit(10) || c == '.' {
                    None
                } else {
                    Some(((x, y), (c, Vec::new())))
                }
            })
        })
        .flatten()
        .collect::<HashMap<_, _>>();

    input.lines().enumerate().for_each(|(y, line)| {
        let runs = line.chars().enumerate().group_by(|(_, c)| c.is_digit(10));
        runs.into_iter()
            .filter_map(|(is_digit, group)| is_digit.then_some(group))
            .for_each(|run| {
                let run = run.collect_vec();

                let begin = run.first().unwrap().0;
                let end = run.last().unwrap().0;

                let number: u32 = run
                    .into_iter()
                    .map(|(_, c)| c)
                    .collect::<String>()
                    .parse()
                    .unwrap();

                let before_begin = begin.saturating_sub(1);
                let after_end = end + 1;
                for cords in (before_begin..=after_end)
                    .map(|x| [(x, y.wrapping_sub(1)), (x, y + 1)])
                    .flatten()
                    .chain([(before_begin, y), (after_end, y)].into_iter())
                {
                    if let Some((_, numbers)) = symbols.get_mut(&cords) {
                        numbers.push(number);
                    }
                }
            })
    });

    let sum: u32 = symbols
        .into_iter()
        .filter_map(|(_, (c, numbers))| {
            (c == '*' && numbers.len() == 2).then(|| numbers.into_iter().product::<u32>())
        })
        .sum();

    println!("{sum}");
}
