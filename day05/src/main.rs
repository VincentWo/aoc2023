#![feature(iterator_try_collect, iter_array_chunks)]
use std::{cmp, convert::Infallible, mem, ops::Range, str::FromStr};

#[derive(Debug)]
struct Rule {
    in_range: Range<i64>,
    transform: i64,
}
impl Rule {
    fn transform(&self, ranges: Vec<Range<i64>>) -> (Vec<Range<i64>>, Vec<Range<i64>>) {
        ranges
            .into_iter()
            .map(|range| {
                let overlap = (cmp::max(range.start, self.in_range.start) + self.transform)
                    ..(cmp::min(range.end, self.in_range.end) + self.transform);

                let before = range.start..cmp::min(self.in_range.start, range.end);
                let after = cmp::max(self.in_range.end, range.start)..range.end;

                (overlap, [before, after])
            })
            .fold(
                (Vec::new(), Vec::new()),
                |(mut transformed, mut not_transformed), (overlap, [before, after])| {
                    if !overlap.is_empty() {
                        transformed.push(overlap);
                    }
                    if !before.is_empty() {
                        not_transformed.push(before);
                    }
                    if !after.is_empty() {
                        not_transformed.push(after);
                    }

                    (transformed, not_transformed)
                },
            )
        // panic!()
        // self.in_range.contains(&id).then(|| id + self.transform)
    }
}

impl FromStr for Rule {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [out_range_start, in_range_start, range_length] = s
            .split_whitespace()
            .map(|s| s.parse())
            .try_collect::<Vec<_>>()
            .unwrap()
            .try_into()
            .unwrap();

        Ok(Rule {
            in_range: in_range_start..(in_range_start + range_length),
            transform: out_range_start - in_range_start,
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Default)]
enum Kind {
    #[default]
    Seed,
    Location,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
}

impl FromStr for Kind {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "soil" => Kind::Soil,
            "fertilizer" => Kind::Fertilizer,
            "water" => Kind::Water,
            "light" => Kind::Light,
            "temperature" => Kind::Temperature,
            "humidity" => Kind::Humidity,
            "location" => Kind::Location,
            "seed" => Kind::Seed,
            unknown => todo!("{unknown}"),
        })
    }
}
struct Map {
    from: Kind,
    to: Kind,
    rules: Vec<Rule>,
}

impl Map {
    fn convert(&self, entry: Entry) -> Entry {
        assert!(entry.kind == self.from);

        let (mut transformed, not_yet_transformed) = self.rules.iter().fold(
            (Vec::new(), entry.ranges),
            |(mut already_transformed, not_yet_transformed), rule| {
                let (newly_transformed, not_yet_transformed) = rule.transform(not_yet_transformed);

                already_transformed.extend(newly_transformed);

                (already_transformed, not_yet_transformed)
            },
        );

        transformed.extend_from_slice(&not_yet_transformed);

        Entry {
            kind: self.to,
            ranges: transformed,
        }
    }
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (from, to) = lines
            .next()
            .unwrap()
            .strip_suffix(" map:")
            .unwrap()
            .split_once("-to-")
            .unwrap();

        let from = from.to_string().parse().unwrap();
        let to = to.to_string().parse().unwrap();

        let rules = lines.map(|r| r.parse().unwrap()).collect();

        Ok(Map { from, to, rules })
    }
}

#[derive(Debug, Default)]
struct Entry {
    kind: Kind,
    ranges: Vec<Range<i64>>,
}
struct Almanac {
    items: Entry,
    maps: Vec<Map>,
}

impl Almanac {
    fn nearest_location(mut self) -> i64 {
        let mut items = mem::take(&mut self.items);

        while items.kind != Kind::Location {
            items = self.convert(items);
        }

        items.ranges.into_iter().map(|r| r.start).min().unwrap()
    }

    fn convert(&self, entry: Entry) -> Entry {
        let map = self.maps.iter().find(|map| map.from == entry.kind).unwrap();
        map.convert(entry)
    }
}

impl FromStr for Almanac {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = s.trim().split("\n\n");
        let ranges = entries
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .array_chunks::<2>()
            .map(|[start, length]| start..(start + length))
            .collect();
        let maps = entries.map(|m| m.parse().unwrap()).collect();

        Ok(Self {
            items: Entry {
                kind: Kind::Seed,
                ranges,
            },
            maps,
        })
    }
}
fn main() {
    let input = include_str!("input");

    let almanac: Almanac = input.parse().unwrap();

    println!("{}", almanac.nearest_location());
}
