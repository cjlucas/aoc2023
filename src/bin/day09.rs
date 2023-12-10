use aoc2023::prelude::parse_lines;

const INPUT: &str = include_str!("../../inputs/day09.txt");

struct History {
    differences: Vec<Vec<i64>>,
}

impl History {
    fn extrapolate_next(&mut self) -> i64 {
        for i in (1..self.differences.len()).rev() {
            let curlast = *self.differences[i].last().unwrap();
            let prevlast = *self.differences[i - 1].last().unwrap();
            self.differences[i - 1].push(curlast + prevlast);
        }

        *self.differences[0].last().unwrap()
    }

    fn extrapolate_prev(&mut self) -> i64 {
        for i in (1..self.differences.len()).rev() {
            let curfirst = self.differences[i][0];
            let prevfirst = self.differences[i - 1][0];
            self.differences[i - 1].insert(0, prevfirst - curfirst);
        }

        self.differences[0][0]
    }
}

impl std::str::FromStr for History {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut history: Vec<Vec<i64>> = vec![s
            .split_whitespace()
            .map(|n| str::parse::<i64>(n).unwrap())
            .collect()];

        while !history.last().unwrap().iter().all(|n| *n == 0) {
            history.push(
                history
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect(),
            );
        }

        Ok(Self {
            differences: history,
        })
    }
}

fn part1(input: &str) -> i64 {
    parse_lines::<History>(input)
        .map(|mut history| history.extrapolate_next())
        .sum()
}

fn part2(input: &str) -> i64 {
    parse_lines::<History>(input)
        .map(|mut history| history.extrapolate_prev())
        .sum()
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
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1842168671);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day09_example.txt");
        assert_eq!(part2(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day09.txt");
        assert_eq!(part2(input), 903);
    }
}
