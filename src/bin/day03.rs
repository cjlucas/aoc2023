// use aoc2023::prelude::*;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day03.txt");

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

                    let Some(c) = schematic.get(&(x as usize, y as usize)) else {
                        continue;
                    };

                    // dbg!(c);

                    // println!("{} {}", *ch, c);

                    if !char::is_digit(*c, 10) && *c != '.' {
                        // dbg!(c);
                        is_adjacent = true;
                    }
                }
            } else if !num.is_empty() {
                let mut n = 0;
                for (pos, digit) in num.iter().rev().enumerate() {
                    let digit = digit.to_digit(10).unwrap() as usize;
                    n += digit * 10usize.pow(pos as u32);
                }

                // println!("({n}, {is_adjacent})");

                if is_adjacent {
                    // dbg!(n);
                    sum += n as u64;
                }

                num = vec![];
                is_adjacent = false;
            }
        }

        if !num.is_empty() {
            let mut n = 0;
            for (pos, digit) in num.iter().rev().enumerate() {
                let digit = digit.to_digit(10).unwrap() as usize;
                n += digit * 10usize.pow(pos as u32);
            }

            // println!("({n}, {is_adjacent})");

            if is_adjacent {
                // dbg!(n);
                sum += n as u64;
            }

            num = vec![];
            is_adjacent = false;
        }
    }

    assert!(num.is_empty());

    sum
}

fn part2(input: &str) -> u64 {
    unimplemented!()
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
        assert_eq!(part1(INPUT), 0);
    }

    // #[test]
    // fn test_part2_example() {
    //     let input = include_str!("../../inputs/day03_example.txt");
    //     assert_eq!(part2(input), 0);
    // }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("../../inputs/day03.txt");
    //     assert_eq!(part2(input), 0);
    // }
}
