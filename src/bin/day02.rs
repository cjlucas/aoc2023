const INPUT: &str = include_str!("../../inputs/day02.txt");

#[derive(Debug)]
struct Game {
    id: u64,
    cube_sets: Vec<CubeSet>,
}

impl std::str::FromStr for Game {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, rest) = s.split_once(':').unwrap();
        let (_, id) = game.split_once(' ').unwrap();
        let id = str::parse(id).unwrap();
        let cube_sets = rest
            .trim()
            .split(';')
            .map(|cube_set| str::parse(cube_set).unwrap())
            .collect();

        Ok(Self { id, cube_sets })
    }
}

#[derive(Debug)]
struct CubeSet {
    num_red: u64,
    num_green: u64,
    num_blue: u64,
}

impl std::str::FromStr for CubeSet {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_set = Self {
            num_red: 0,
            num_green: 0,
            num_blue: 0,
        };

        for cube in s.split(',') {
            let (cnt, color) = cube.trim().split_once(' ').unwrap();
            let cnt = str::parse(cnt).unwrap();

            match color {
                "red" => cube_set.num_red = cnt,
                "green" => cube_set.num_green = cnt,
                "blue" => cube_set.num_blue = cnt,
                _ => unreachable!(),
            }
        }

        Ok(cube_set)
    }
}

fn part1(input: &str) -> u64 {
    const MAX_RED: u64 = 12;
    const MAX_GREEN: u64 = 13;
    const MAX_BLUE: u64 = 14;

    let mut sum = 0;

    for line in input.lines() {
        let game: Game = str::parse(line).unwrap();

        let num_red = game.cube_sets.iter().map(|set| set.num_red).max().unwrap();
        let num_green = game
            .cube_sets
            .iter()
            .map(|set| set.num_green)
            .max()
            .unwrap();
        let num_blue = game.cube_sets.iter().map(|set| set.num_blue).max().unwrap();

        if num_red <= MAX_RED && num_green <= MAX_GREEN && num_blue <= MAX_BLUE {
            sum += game.id;
        }
    }

    sum
}

fn part2(input: &str) -> u64 {
    let mut power = 0;

    for line in input.lines() {
        let game: Game = str::parse(line).unwrap();

        let num_red = game.cube_sets.iter().map(|set| set.num_red).max().unwrap();
        let num_green = game
            .cube_sets
            .iter()
            .map(|set| set.num_green)
            .max()
            .unwrap();
        let num_blue = game.cube_sets.iter().map(|set| set.num_blue).max().unwrap();

        power += num_red * num_green * num_blue
    }

    power
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
