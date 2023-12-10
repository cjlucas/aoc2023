use aoc2023::prelude::*;

const INPUT: &str = include_str!("../../inputs/day09.txt");

fn part1(input: &str) -> i64 {
    let mut foo: Vec<Vec<i64>> = vec![input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| str::parse::<i64>(n).unwrap())
        .collect()];

    while !foo.last().unwrap().iter().all(|n| *n == 0) {
        foo.push(
            foo.last()
                .unwrap()
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect(),
        );
    }

    for i in (1..foo.len()).rev() {
        foo[i - 1].push(foo[foo.len() - 1][-1] + foo[i][-1]);
    }

    println!("foo: {:?}", foo);

    unimplemented!()
}

fn part2(input: &str) -> i64 {
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
        let input = include_str!("../../inputs/day09_example.txt");
        assert_eq!(part1(input), 8);
    }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(INPUT), 2331);
    // }

    // #[test]
    // fn test_part2_example() {
    //     let input = include_str!("../../inputs/day09_example.txt");
    //     assert_eq!(part2(input), 2286);
    // }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("../../inputs/day09.txt");
    //     assert_eq!(part2(input), 71585);
    // }
}
