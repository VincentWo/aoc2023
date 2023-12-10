#![feature(array_windows)]
use std::{convert::Infallible, mem, str::FromStr};

#[derive(Debug)]
struct Report {
    histories: Vec<Vec<i64>>,
}

impl Report {
    fn extrapolated_sum(&self) -> i64 {
        self.histories
            .iter()
            .map(|history| {
                let mut first_values = vec![history.first().copied().unwrap()];

                let mut differences: Vec<_> = history.clone();

                while differences.iter().any(|a| *a != 0) {
                    let new_differences: Vec<_> = mem::take(&mut differences)
                        .array_windows::<2>()
                        .map(|[a, b]| b - a)
                        .collect();

                    first_values.push(new_differences.first().copied().unwrap());

                    differences = new_differences;
                }

                println!("{first_values:?}");

                first_values
                    .into_iter()
                    .rev()
                    .reduce(|last_value, current| current - last_value)
                    .unwrap()
            })
            .sum()
    }
}

impl FromStr for Report {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let histories = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();

        Ok(Report { histories })
    }
}
fn main() {
    let input = include_str!("input").trim();

    let report: Report = input.parse().unwrap();

    let sum = report.extrapolated_sum();

    println!("{sum}");
}
