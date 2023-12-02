const INPUT: &str = include_str!("../../inputs/day01.txt");

fn solve<const N: usize>(input: &str, words: [(&str, u64); N]) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let a = words
            .iter()
            .filter_map(|(word, val)| line.find(word).map(|idx| (idx, val)))
            .min()
            .unwrap()
            .1;

        let b = words
            .iter()
            .filter_map(|(word, val)| line.rfind(word).map(|idx| (idx, val)))
            .max()
            .unwrap()
            .1;

        sum += (a * 10) + b;
    }

    sum
}

fn part1(input: &str) -> u64 {
    solve(
        input,
        [
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ],
    )
}

fn part2(input: &str) -> u64 {
    solve(
        input,
        [
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ],
    )
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
        let input = include_str!("../../inputs/day01p1_example.txt");
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 54634);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day01p2_example.txt");
        assert_eq!(part2(input), 281);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 53855);
    }
}
