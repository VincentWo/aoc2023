#![feature(isqrt)]

use std::{cmp, convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Race {
    time: i64,
    record: u64,
}

impl Race {
    fn margin_of_error(&self) -> u64 {
        let p_halves = self.time as f64 / 2.0;
        let sqrt_term = ((p_halves as f64).powi(2) - self.record as f64).sqrt();
        let [begin, end] = [
            cmp::max((p_halves - sqrt_term).floor() as i64 + 1, 0),
            cmp::min((p_halves + sqrt_term).ceil() as i64 - 1, self.time),
        ];

        if begin <= end {
            end.abs_diff(begin) + 1
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct RaceData {
    races: Vec<Race>,
}

impl RaceData {
    fn product_of_error_margin(&self) -> u64 {
        self.races
            .iter()
            .map(|r| r.margin_of_error())
            .inspect(|margin| {
                // dbg!(margin);
            })
            .product()
    }
}

impl FromStr for RaceData {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (times, records) = s.split_once('\n').unwrap();
        let time: String = times
            .strip_prefix("Time:")
            .unwrap()
            .trim_start()
            .split_whitespace()
            .collect();
        // .map(|s| s.parse().unwrap());
        let record: String = records
            .strip_prefix("Distance:")
            .unwrap()
            .trim_start()
            .split_whitespace()
            .collect();
        // .map(|s| s.parse().unwrap());

        // let races = times
        //     .zip(records)
        //     .map(|(time, record)| Race { time, record })
        //     .collect();

        Ok(RaceData {
            races: vec![Race {
                time: time.parse().unwrap(),
                record: record.parse().unwrap(),
            }],
        })
    }
}

fn main() {
    let input = include_str!("input");

    let data: RaceData = input.parse().unwrap();

    println!("{}", data.product_of_error_margin());
}
