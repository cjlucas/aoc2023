const INPUT: &'static str = include_str!("../../inputs/day01.txt");

fn part1(input: &str) -> u64 {
    let mut sum = 0;

    for line in input.lines() {
        let a = line
            .chars()
            .find(|ch| ch.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let b = line
            .chars()
            .rev()
            .find(|ch| ch.is_numeric())
            .unwrap()
            .to_digit(10)
            .unwrap();
        sum += (a * 10) + b;
    }

    sum.into()
}

fn part2(input: &str) -> u64 {
    let words = vec![
        ("1", 1u64),
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
    ];

    let mut sum = 0;

    for line in input.lines() {
        let hits: Vec<_> = words
            .iter()
            .filter_map(|(word, val)| line.find(word).map(|idx| (idx, val)))
            .collect();

        let a = hits.iter().min().unwrap();

        let hits: Vec<_> = words
            .iter()
            .filter_map(|(word, val)| line.rfind(word).map(|idx| (idx, val)))
            .collect();

        let b = hits.iter().max().unwrap();
        sum += (a.1 * 10) + b.1;
    }

    sum.into()
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
        const INPUT: &'static str = include_str!("../../inputs/day01p1_example.txt");
        assert_eq!(part1(INPUT), 142);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 54634);
    }

    #[test]
    fn test_part2_example() {
        const INPUT: &'static str = include_str!("../../inputs/day01p2_example.txt");
        assert_eq!(part2(INPUT), 281);
    }

    #[test]
    fn test_part2() {
        const INPUT: &'static str = include_str!("../../inputs/day01.txt");
        assert_eq!(part2(INPUT), 53855);
    }
}
