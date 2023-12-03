// use aoc2023::prelude::*;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day03.txt");

struct Grid {
    num_rows: usize,
    num_cols: usize,
    tiles: HashMap<(usize, usize), char>,
}

impl std::str::FromStr for Grid {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = HashMap::<(usize, usize), char>::new();
        let mut num_rows = 0;
        let mut num_cols = 0;
        for (row_idx, row) in s.lines().enumerate() {
            num_rows += 1;
            num_cols = row.len();

            for (col_idx, ch) in row.chars().enumerate() {
                tiles.insert((col_idx, row_idx), ch);
            }
        }

        Ok(Self {
            num_rows,
            num_cols,
            tiles,
        })
    }
}

fn num_from_digits(chars: &[char]) -> u64 {
    chars
        .iter()
        .rev()
        .enumerate()
        .map(|(pos, digit)| {
            let digit = digit.to_digit(10).unwrap() as u64;
            digit * 10u64.pow(pos as u32)
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    let mut schematic = HashMap::<(usize, usize), char>::new();

    let mut num_rows = 0;
    let mut num_cols = 0;
    for (row_idx, row) in input.lines().enumerate() {
        num_rows += 1;
        num_cols = row.len();

        for (col_idx, ch) in row.chars().enumerate() {
            schematic.insert((col_idx, row_idx), ch);
        }
    }

    let mut sum = 0;

    let mut num = vec![];
    let mut is_adjacent = false;
    for j in 0..num_rows {
        for i in 0..num_cols {
            let ch = *schematic.get(&(i, j)).unwrap();
            if char::is_digit(ch, 10) {
                num.push(ch);
                let adjacent_tiles = vec![
                    (-1i64, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];

                for (x, y) in adjacent_tiles {
                    let i = i as i64;
                    let j = j as i64;

                    let Some(x) = i.checked_add(x) else {
                        continue;
                    };
                    let Some(y) = j.checked_add(y) else {
                        continue;
                    };

                    let Some(&c) = schematic.get(&(x as usize, y as usize)) else {
                        continue;
                    };

                    if !char::is_digit(c, 10) && c != '.' {
                        is_adjacent = true;
                    }
                }
            } else if !num.is_empty() {
                if is_adjacent {
                    sum += num_from_digits(&num[..]);
                }

                num = vec![];
                is_adjacent = false;
            }
        }

        if !num.is_empty() {
            if is_adjacent {
                sum += num_from_digits(&num[..]);
            }

            num = vec![];
            is_adjacent = false;
        }
    }

    sum
}

fn part2(input: &str) -> u64 {
    let mut schematic = HashMap::<(usize, usize), char>::new();

    let mut num_rows = 0;
    let mut num_cols = 0;
    for (row_idx, row) in input.lines().enumerate() {
        num_rows += 1;
        num_cols = row.len();

        for (col_idx, ch) in row.chars().enumerate() {
            schematic.insert((col_idx, row_idx), ch);
        }
    }

    let mut sum = 0;

    // key is coord of *, value is list of numbers adjacent to *
    let mut gear_nums = HashMap::<(usize, usize), Vec<u64>>::new();

    let mut num = vec![];
    let mut is_adjacent = None;
    for j in 0..num_rows {
        for i in 0..num_cols {
            let ch = schematic.get(&(i, j)).unwrap();
            // dbg!(ch);

            if char::is_digit(*ch, 10) {
                num.push(ch);
                let adjacent_tiles = vec![
                    (-1i64, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];

                for (x, y) in adjacent_tiles {
                    let i = i as i64;
                    let j = j as i64;

                    // dbg!((i, j, x, y));

                    let Some(x) = i.checked_add(x) else {
                        continue;
                    };
                    let Some(y) = j.checked_add(y) else {
                        continue;
                    };

                    if x < 0 || y < 0 {
                        continue;
                    }
                    // dbg!((x, y));

                    let coord = (x as usize, y as usize);
                    let Some(c) = schematic.get(&coord) else {
                        continue;
                    };

                    // dbg!(c);

                    // println!("{} {}", *ch, c);

                    if *c == '*' {
                        // dbg!(c);
                        is_adjacent = Some(coord);
                    }
                }
            } else if !num.is_empty() {
                let mut n = 0;
                for (pos, digit) in num.iter().rev().enumerate() {
                    let digit = digit.to_digit(10).unwrap() as usize;
                    n += digit * 10usize.pow(pos as u32);
                }

                // println!("({n}, {is_adjacent})");

                if let Some(coord) = is_adjacent {
                    let nums = gear_nums.entry(coord).or_default();
                    nums.push(n as u64);
                }

                num = vec![];
                is_adjacent = None;
            }
        }

        if !num.is_empty() {
            let mut n = 0;
            for (pos, digit) in num.iter().rev().enumerate() {
                let digit = digit.to_digit(10).unwrap() as usize;
                n += digit * 10usize.pow(pos as u32);
            }

            // println!("({n}, {is_adjacent})");

            if let Some(coord) = is_adjacent {
                let nums = gear_nums.entry(coord).or_default();
                nums.push(n as u64);
            }

            num = vec![];
            is_adjacent = None;
        }
    }

    for (_, v) in gear_nums {
        if v.len() > 1 {
            sum += v.iter().product::<u64>()
        }
    }

    assert!(num.is_empty());

    sum
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
        let input = include_str!("../../inputs/day03_example.txt");
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 530849);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day03_example.txt");
        assert_eq!(part2(input), 467835);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day03.txt");
        assert_eq!(part2(input), 84900879);
    }
}
