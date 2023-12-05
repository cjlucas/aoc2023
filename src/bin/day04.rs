use aoc2023::prelude::*;

use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day04.txt");

struct Card {
    id: u64,
    winning_numbers: HashSet<u64>,
    numbers: HashSet<u64>,
}

impl Card {
    fn num_matches(&self) -> u64 {
        self.winning_numbers.intersection(&self.numbers).count() as u64
    }

    fn score(&self) -> u64 {
        let num_matches = self.num_matches();
        if num_matches > 0 {
            1 << (num_matches - 1)
        } else {
            0
        }
    }
}

impl std::str::FromStr for Card {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_id, rest) = s.split_once(':').unwrap();
        let (_, id) = card_id.split_once(' ').unwrap();
        let id = str::parse(id.trim()).unwrap();

        let (winning_numbers, numbers) = rest.trim().split_once('|').unwrap();
        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|n| str::parse(n).unwrap())
            .collect();

        let numbers = numbers
            .split_whitespace()
            .map(|n| str::parse(n).unwrap())
            .collect();

        Ok(Self {
            id,
            winning_numbers,
            numbers,
        })
    }
}

fn part1(input: &str) -> u64 {
    parse_lines::<Card>(input).map(|card| card.score()).sum()
}

fn part2(input: &str) -> u64 {
    let cards: Vec<_> = parse_lines::<Card>(input).collect();
    let mut num_cards = HashMap::<u64, u64>::new();

    for card in cards {
        let n = num_cards.entry(card.id).or_insert_with(|| 0);
        *n += 1;

        for _ in 0..*n {
            for id in card.id..=(card.id + card.num_matches()) {
                let cnt = num_cards.entry(id).or_default();
                *cnt += 1;
            }
        }
    }

    num_cards.values().sum::<u64>() / 2
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../../inputs/day04_example.txt");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 24160);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day04_example.txt");
        assert_eq!(part2(input), 30);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day04.txt");
        assert_eq!(part2(input), 5659035);
    }
}
