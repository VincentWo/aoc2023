use std::{collections::HashSet, convert::Infallible, str::FromStr};

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}
impl Card {
    fn points(&self) -> usize {
        let winning_count = self.winning_numbers.intersection(&self.numbers).count();

        winning_count
    }
}

struct Game {
    cards: Vec<(Card, Option<u32>)>,
}

impl Game {
    fn value(&mut self) -> u32 {
        (1..=self.cards.len())
            .map(|id| self.value_of_card(id))
            .sum()
    }

    fn value_of_card(&mut self, id: usize) -> u32 {
        let (card, value) = &mut self.cards[id - 1];
        let points = card.points() as usize;
        // println!("{id}: {points}");

        if let Some(value) = value {
            *value
        } else {
            let value = ((id + 1)..(id + 1 + points))
                .map(|id| self.value_of_card(id))
                .sum::<u32>()
                + 1;

            self.cards[id - 1].1 = Some(value);
            value
        }
    }
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Card").unwrap().trim();
        let (id, rest) = s.split_once(':').unwrap();
        let id = id.parse().unwrap();

        let (winning, numbers) = rest.split_once('|').unwrap();

        let [winning_numbers, numbers] = [winning, numbers].map(|list| {
            list.trim()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect()
        });

        Ok(Card {
            id,
            winning_numbers,
            numbers,
        })
    }
}
fn main() {
    let input = include_str!("input");

    let cards = input
        .lines()
        .map(|line| {
            (line.parse::<Card>().unwrap(), None)

            // card.points()
        })
        .collect::<Vec<_>>();

    let mut game = Game { cards };

    println!("{}", game.value());
}
