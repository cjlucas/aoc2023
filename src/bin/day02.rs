const INPUT: &str = include_str!("../../inputs/day02.txt");

fn part1(input: &str) -> u64 {
    const MAX_RED: u64 = 12;
    const MAX_GREEN: u64 = 13;
    const MAX_BLUE: u64 = 14;

    let mut sum = 0;

    for (line_num, line) in input.lines().enumerate() {
        let (_, sets) = line.split_once(':').unwrap();
        let sets = sets.split(";").collect::<Vec<_>>();

        let mut num_red = 0;
        let mut num_green = 0;
        let mut num_blue = 0;

        for set in sets {
            for cube in set.split(',') {
                let (cnt, color) = cube.trim().split_once(' ').unwrap();
                let cnt = u64::from_str_radix(cnt, 10).unwrap();

                if color == "red" && cnt > num_red {
                    num_red = cnt;
                }
                if color == "green" && cnt > num_green {
                    num_green = cnt;
                }
                if color == "blue" && cnt > num_blue {
                    num_blue = cnt;
                }
            }
        }

        if num_red <= MAX_RED && num_green <= MAX_GREEN && num_blue <= MAX_BLUE {
            sum += line_num + 1;
        }
    }

    sum as u64
}

fn part2(input: &str) -> u64 {
    let mut power = 0;

    for line in input.lines() {
        let (_, sets) = line.split_once(':').unwrap();
        let sets = sets.split(";").collect::<Vec<_>>();

        let mut num_red = 0;
        let mut num_green = 0;
        let mut num_blue = 0;

        for set in sets {
            for cube in set.split(',') {
                let (cnt, color) = cube.trim().split_once(' ').unwrap();
                let cnt = u64::from_str_radix(cnt, 10).unwrap();

                if color == "red" && cnt > num_red {
                    num_red = cnt;
                }
                if color == "green" && cnt > num_green {
                    num_green = cnt;
                }
                if color == "blue" && cnt > num_blue {
                    num_blue = cnt;
                }
            }
        }

        power += num_red * num_green * num_blue;
    }

    power as u64
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
        let input = include_str!("../../inputs/day02_example.txt");
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 2331);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day02_example.txt");
        assert_eq!(part2(input), 2286);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day02.txt");
        assert_eq!(part2(input), 71585);
    }
}
