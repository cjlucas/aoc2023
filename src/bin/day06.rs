const INPUT: &str = include_str!("../../inputs/day06.txt");

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_races1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();
    let times = times
        .split_whitespace()
        .map(|time| str::parse::<u64>(time).unwrap());
    let distances = distances
        .split_whitespace()
        .map(|time| str::parse::<u64>(time).unwrap());

    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_races2(input: &str) -> Race {
    let mut lines = input.lines();
    let (_, times) = lines.next().unwrap().split_once(':').unwrap();
    let (_, distances) = lines.next().unwrap().split_once(':').unwrap();
    let time = times.split_whitespace().fold(String::new(), |mut s, t| {
        s.push_str(t.trim());
        s
    });

    let distance = distances
        .split_whitespace()
        .fold(String::new(), |mut s, t| {
            s.push_str(t.trim());
            s
        });

    Race {
        time: str::parse(&time).unwrap(),
        distance: str::parse(&distance).unwrap(),
    }
}

fn part1(input: &str) -> u64 {
    let races = parse_races1(input);

    let mut foo = vec![];
    for race in races {
        let mut num_winners = 0u64;
        for time_to_hold_down in 0..race.time {
            let time_to_race = race.time - time_to_hold_down;
            if time_to_hold_down * time_to_race > race.distance {
                num_winners += 1;
            }
        }

        foo.push(num_winners)
    }

    foo.iter().product()
}

fn part2(input: &str) -> u64 {
    let race = parse_races2(input);

    let mut num_winners = 0u64;
    for time_to_hold_down in 0..race.time {
        let time_to_race = race.time - time_to_hold_down;
        if time_to_hold_down * time_to_race > race.distance {
            num_winners += 1;
        }
    }

    num_winners
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day06_part1_example() {
        let input = include_str!("../../inputs/day06_example.txt");
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 771628);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day06_example.txt");
        assert_eq!(part2(input), 71503);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day06.txt");
        assert_eq!(part2(input), 27363861);
    }
}
