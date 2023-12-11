const INPUT: &str = include_str!("../../inputs/day11.txt");

fn part1(input: &str) -> i64 {
    unreachable!()
}

fn part2(input: &str) -> i64 {
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
        let input = include_str!("../../inputs/day11_example.txt");
        assert_eq!(part1(input), 374);
    }

    // #[test]
    // fn test_part1() {
    //     assert_eq!(part1(INPUT), -1);
    // }

    // #[test]
    // fn test_part2_example() {
    //     let input = include_str!("../../inputs/day11_example.txt");
    //     assert_eq!(part2(input), -1);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), -1);
    // }
}
