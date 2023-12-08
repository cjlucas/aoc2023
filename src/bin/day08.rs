use std::collections::HashMap;

use aoc2023::prelude::*;

const INPUT: &str = include_str!("../../inputs/day08.txt");

enum Direction {
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => unreachable!(),
        }
    }
}

fn part1(input: &str) -> u64 {
    let (directions, network) = input.split_once("\n\n").unwrap();
    let directions = directions
        .chars()
        .map(|c| str::parse::<Direction>(&c.to_string()).unwrap())
        .cycle();

    let mut network_map = HashMap::<String, (String, String)>::new();

    for line in network.lines() {
        let (key, val) = line.split_once('=').unwrap();
        let key = key.trim().to_string();
        let (left, right) = val.split_once(',').unwrap();
        let left = left.trim()[1..].to_string();
        let right = right.trim()[..right.len() - 2].to_string();

        network_map.insert(key, (left, right));
    }

    let mut cur_node = "AAA".to_string();

    let mut steps = 0;

    for dir in directions {
        if cur_node == "ZZZ" {
            break;
        }

        let (left, right) = network_map.get(&cur_node).unwrap();

        cur_node = match dir {
            Direction::Left => left.to_string(),
            Direction::Right => right.to_string(),
        };

        steps += 1;
    }

    steps
}

fn part2(input: &str) -> u64 {
    let (directions, network) = input.split_once("\n\n").unwrap();
    let directions = directions
        .chars()
        .map(|c| str::parse::<Direction>(&c.to_string()).unwrap())
        .cycle();

    let mut network_map = HashMap::<String, (String, String)>::new();

    for line in network.lines() {
        let (key, val) = line.split_once('=').unwrap();
        let key = key.trim().to_string();
        let (left, right) = val.split_once(',').unwrap();
        let left = left.trim()[1..].to_string();
        let right = right.trim()[..right.len() - 2].to_string();

        network_map.insert(key, (left, right));
    }

    let mut cur_nodes: Vec<_> = network_map
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect();

    dbg!(&network_map);

    dbg!(cur_nodes.len());
    dbg!(&cur_nodes);
    // panic!();

    let mut steps = 0;

    for dir in directions {
        if steps % 100000 == 0 {
            println!("{}", steps);
        }

        for node in &mut cur_nodes {
            let (left, right) = network_map.get(node).unwrap();

            *node = match dir {
                Direction::Left => left.to_string(),
                Direction::Right => right.to_string(),
            };
        }

        steps += 1;

        if cur_nodes.iter().all(|node| node.ends_with('Z')) {
            break;
        }
    }

    steps
}

fn main() {
    // println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../../inputs/day08_example1.txt");
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part1_example2() {
        let input = include_str!("../../inputs/day08_example2.txt");
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 17873);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day08p2_example.txt");
        assert_eq!(part2(input), 6);
    }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("../../inputs/day08.txt");
    //     assert_eq!(part2(input), 71585);
    // }
}
