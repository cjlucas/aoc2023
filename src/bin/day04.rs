use aoc2023::prelude::*;

use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day04.txt");

struct Card {
    id: u64,
    winning_numbers: HashSet<u64>,
    numbers: HashSet<u64>,
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
    parse_lines::<Card>(input)
        .map(|card| {
            let num_matches = card.winning_numbers.intersection(&card.numbers).count();

            if num_matches > 0 {
                1 << (num_matches - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    unreachable!()
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

    // #[test]
    // fn test_part2_example() {
    //     let input = include_str!("../../inputs/day04_example.txt");
    //     assert_eq!(part2(input), 2286);
    // }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("../../inputs/day04.txt");
    //     assert_eq!(part2(input), 71585);
    // }
}
