use std::{cmp, convert::Infallible, iter::Sum, ops::Add, str::FromStr};

#[derive(Debug)]
struct Game {
    id: u32,
    runs: Vec<Run>,
}
impl Game {
    fn is_possible_with(&self, bag: &Bag) -> bool {
        self.runs.iter().all(|run| run.is_possible_with(bag))
    }

    fn minimal_bag(&self) -> Bag {
        self.runs.iter().fold(Bag::default(), |bag, run| Bag {
            red: cmp::max(bag.red, run.red),
            green: cmp::max(bag.green, run.green),
            blue: cmp::max(bag.blue, run.blue),
        })
    }
}

#[derive(Debug, Default)]
struct Run {
    blue: u32,
    red: u32,
    green: u32,
}

#[derive(Debug, Default)]
struct Bag {
    blue: u32,
    red: u32,
    green: u32,
}

impl Bag {
    fn power(&self) -> u32 {
        self.blue * self.red * self.green
    }
}

impl Add for Run {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Run {
            blue: self.blue + rhs.blue,
            red: self.red + rhs.red,
            green: self.green + rhs.green,
        }
    }
}

impl Sum for Run {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or_default()
    }
}

impl Run {
    fn only_blue(count: u32) -> Self {
        Run {
            blue: count,
            red: 0,
            green: 0,
        }
    }

    fn only_red(count: u32) -> Self {
        Run {
            red: count,
            blue: 0,
            green: 0,
        }
    }

    fn only_green(count: u32) -> Self {
        Run {
            green: count,
            blue: 0,
            red: 0,
        }
    }

    fn is_possible_with(&self, bag: &Bag) -> bool {
        let &Run { red, green, blue } = self;

        red <= bag.red && green <= bag.green && blue <= bag.blue
    }
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").unwrap();
        let (id, rest) = s.split_once(":").unwrap();

        let runs = rest
            .split(";")
            .map(|run| {
                run.split(",")
                    .map(|drawing| {
                        let drawing = drawing.trim();
                        let (count, colour) = drawing.split_once(" ").unwrap();
                        let count: u32 = count.parse().unwrap();
                        // let colour: Colour = colour.parse().unwrap();

                        match colour {
                            "green" => Run::only_green(count),
                            "red" => Run::only_red(count),
                            "blue" => Run::only_blue(count),
                            unknown => todo!("{unknown}"),
                        }
                    })
                    .sum()
            })
            .collect();

        Ok(Game {
            id: id.parse().unwrap(),
            runs,
        })
    }
}

fn main() {
    let input = include_str!("input");

    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let out: u32 = input
        .lines()
        .map(|game| dbg!(game.parse::<Game>().unwrap()).minimal_bag().power())
        .sum();

    println!("{out:#?}");
}
