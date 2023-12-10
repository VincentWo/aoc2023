use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::HashMap,
    convert::Infallible,
    fmt::{Debug, Write},
    str::FromStr,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Joker,
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => 'T',
            Card::Joker => 'J',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
        };

        f.write_char(c)
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Joker,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            unsupported => panic!("Unexpected card: {unsupported}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOf,
    FullHouse,
    FourOf,
    FiveOf,
}

#[derive(PartialEq, Eq, Clone)]
struct Hand {
    cards: [Card; 5],
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Hand { \"")?;

        for card in self.cards {
            write!(f, "{:?}", card)?;
        }

        f.write_str("\" }")
    }
}

impl Hand {
    fn kind(&self) -> HandKind {
        let mut counts = HashMap::<Card, u32>::new();

        for card in self.cards {
            *counts.entry(card).or_default() += 1;
        }

        let joker_count = counts.remove(&Card::Joker).unwrap_or_default();
        let mut counts: Vec<_> = counts.into_values().collect();
        counts.sort_unstable_by_key(|&c| Reverse(c));

        match (
            counts.get(0).copied().unwrap_or_default() + joker_count,
            counts.get(1).copied().unwrap_or_default(),
        ) {
            (5, _) => HandKind::FiveOf,
            (4, _) => HandKind::FourOf,
            (3, 2) => HandKind::FullHouse,
            (3, _) => HandKind::ThreeOf,
            (2, 2) => HandKind::TwoPairs,
            (2, _) => HandKind::Pair,
            (1, _) => HandKind::HighCard,
            should_not_happen => unreachable!("{should_not_happen:?}"),
        }
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| c.into())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Hand { cards })
    }
}

#[cfg(test)]
mod test {
    use crate::{Hand, HandKind};

    #[test]
    fn parsing() {
        let inputs = [
            ("32T3K", HandKind::Pair),
            ("T55J5", HandKind::FourOf),
            ("KK677", HandKind::TwoPairs),
            ("KTJJT", HandKind::FourOf),
            ("QQQJA", HandKind::FourOf),
        ]
        .map(|(hand, kind)| (hand.parse::<Hand>().unwrap(), kind));

        for (hand, kind) in inputs {
            assert_eq!(hand.kind(), kind, "{hand:?} did not evaluate as {kind:?}",);
        }
    }

    #[test]
    fn ordering() {
        let inputs = [
            ("32T3K", 1),
            ("KK677", 2),
            ("T55J5", 3),
            ("QQQJA", 4),
            ("KTJJT", 5),
        ]
        .map(|(hand, rank)| (hand.parse::<Hand>().unwrap(), rank));

        for (hand, rank) in &inputs {
            for (other, other_rank) in &inputs {
                assert_eq!(hand.clone().cmp(other), rank.cmp(other_rank));
                assert_eq!(other.clone().cmp(hand), other_rank.cmp(rank));
            }
        }
    }

    #[test]
    fn test_ordering() {
        assert!(HandKind::Pair <= HandKind::ThreeOf);
        assert!(HandKind::TwoPairs <= HandKind::ThreeOf);
        assert!(HandKind::ThreeOf == HandKind::ThreeOf);
        assert!(HandKind::ThreeOf <= HandKind::FourOf);
        assert!(HandKind::ThreeOf <= HandKind::FiveOf);
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind()
            .cmp(&other.kind())
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

#[derive(Debug)]
struct Game {
    games: Vec<(Hand, u32)>,
}
impl Game {
    fn total_winnings(&self) -> u32 {
        self.games
            .iter()
            .enumerate()
            .inspect(|(i, (hand, bid))| {
                println!(
                    "{hand:?} Rank: {:rank_width$} * {bid:4} (bid) = {:6}",
                    i + 1,
                    (*i as u32 + 1) * bid,
                    rank_width = (self.games.len() + 1).ilog10() as usize + 1,
                );
            })
            .map(|(i, (_, bid))| (i as u32 + 1) * bid)
            .sum()
    }
}

impl FromStr for Game {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games = s
            .lines()
            .map(|l| {
                let (hand, bid) = l.split_once(' ').unwrap();

                (hand.parse::<Hand>().unwrap(), bid.parse().unwrap())
            })
            .sorted_unstable_by(|(a, _), (b, _)| a.cmp(b))
            .collect();

        Ok(Game { games })
    }
}

fn main() {
    let input = include_str!("input").trim();

    let game: Game = input.parse().unwrap();

    let total_winnings = game.total_winnings();

    println!("Total winnings: {total_winnings}");
}
