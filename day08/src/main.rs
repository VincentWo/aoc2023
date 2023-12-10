#![feature(map_try_insert)]

use std::{
    collections::HashMap,
    convert::Infallible,
    str::{self, FromStr},
};

use num::integer::lcm;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'l' | 'L' => Direction::Left,
            'r' | 'R' => Direction::Right,
            uncovered => todo!("{uncovered:?}"),
        }
    }
}

struct Node {
    name: [u8; 3],
    left: [u8; 3],
    right: [u8; 3],
}

impl std::hash::Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = str::from_utf8(&self.name).unwrap();
        let left = str::from_utf8(&self.left).unwrap();
        let right = str::from_utf8(&self.right).unwrap();

        write!(f, "{name} = ({left}, {right})")
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl FromStr for Node {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, connections) = s.split_once(" = ").unwrap();
        let (left, right) = connections
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();

        let name = name.as_bytes().try_into().unwrap();
        let left = left.as_bytes().try_into().unwrap();
        let right = right.as_bytes().try_into().unwrap();

        Ok(Node { name, left, right })
    }
}

#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,

    network: Vec<Node>,
}

impl Input {
    fn find_node(&self, name: &[u8; 3]) -> &Node {
        let i = self
            .network
            .binary_search_by(|node| node.name.cmp(name))
            .unwrap();

        &self.network[i]
    }
    fn required_steps(&self) -> usize {
        let starting_nodes: Vec<_> = self
            .network
            .iter()
            .filter(|node| node.name[2] == b'A')
            .collect();

        starting_nodes
            .iter()
            .copied()
            .map(|node| {
                let mut already_visited = HashMap::new();

                let mut current_node = node;
                let mut steps = 0;
                let mut current_direction_step = 0;

                'outer: loop {
                    current_direction_step = 0;
                    for (i, direction) in self.directions.iter().enumerate() {
                        if already_visited
                            .try_insert((i, current_node), steps)
                            .is_err()
                        {
                            break 'outer;
                        }
                        current_node = self.find_node(match direction {
                            Direction::Left => &current_node.left,
                            Direction::Right => &current_node.right,
                        });

                        steps += 1;
                        current_direction_step += 1;
                    }
                }
                let loop_start = already_visited[&(current_direction_step, current_node)];
                let loop_end = steps;

                let [critical_point]: [usize; 1] = already_visited
                    .into_iter()
                    .filter_map(|((_, node), step)| {
                        (node.name[2] == b'Z' && (loop_start..loop_end).contains(&step))
                            .then_some(step)
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();

                assert_eq!(critical_point, loop_end - loop_start);

                critical_point
            })
            .reduce(|a, b| lcm(a, b))
            .unwrap()

        // println!("{cycles:?}");

        // for (i, direction) in std::iter::repeat(&self.directions).flatten().enumerate() {
        //     let mut all_end_with_z = true;
        //     for current_node in &mut current_nodes {
        //         let next_name = &match direction {
        //             Direction::Left => current_node.left,
        //             Direction::Right => current_node.right,
        //         };
        //         *current_node = self.find_node(next_name);

        //         if next_name[2] != b'Z' {
        //             println!("{current_node:?}, {}", i + 1)
        //         }
        //     }

        //     // if all_end_with_z && false {
        //         return i + 1;
        //     }
        // }

        // unreachable!()
    }
}

impl FromStr for Input {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (directions, network) = s.split_once("\n\n").unwrap();

        let directions = directions.chars().map(|c| c.into()).collect();

        let mut network: Vec<_> = network.lines().map(|l| l.parse().unwrap()).collect();

        network.sort_unstable();

        Ok(Self {
            directions,
            network,
        })
    }
}

fn main() {
    let input = include_str!("input");

    let input: Input = input.parse().unwrap();

    println!("{}", input.required_steps());
}
