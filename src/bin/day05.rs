const INPUT: &str = include_str!("../../inputs/day05.txt");

#[derive(Debug, Default)]
struct Almanac {
    // seeds: Vec<u64>,
    seed_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}

fn parse_almanac(input: &str) -> (Vec<u64>, Almanac) {
    let mut almanac = Almanac::default();

    let mut lines = input.lines();
    let (_, seeds) = lines.next().unwrap().split_once(':').unwrap();
    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .map(|seed| str::parse(seed).unwrap())
        .collect();

    lines.next();
    lines.next(); // seed-to-soil map:
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let n: Vec<_> = line
            .split_whitespace()
            .map(|n| str::parse::<u64>(n).unwrap())
            .collect();

        let dst_start = n[0];
        let src_start = n[1];
        let range_len = n[2];

        almanac.seed_to_soil.push((dst_start, src_start, range_len));
    }

    dbg!(lines.next()); // soil-to-fertilizer map:
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let n: Vec<_> = line
            .split_whitespace()
            .map(|n| str::parse::<u64>(n).unwrap())
            .collect();

        let dst_start = n[0];
        let src_start = n[1];
        let range_len = n[2];

        almanac
            .soil_to_fertilizer
            .push((dst_start, src_start, range_len));
    }

    dbg!(lines.next()); // fertilizer-to-water map:
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let n: Vec<_> = line
            .split_whitespace()
            .map(|n| str::parse::<u64>(n).unwrap())
            .collect();

        let dst_start = n[0];
        let src_start = n[1];
        let range_len = n[2];

        almanac
            .fertilizer_to_water
            .push((dst_start, src_start, range_len));
    }

    dbg!(lines.next()); // water-to-light map:
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let n: Vec<_> = line
            .split_whitespace()
            .map(|n| str::parse::<u64>(n).unwrap())
            .collect();

        let dst_start = n[0];
        let src_start = n[1];
        let range_len = n[2];

        almanac
            .water_to_light
            .push((dst_start, src_start, range_len));
    }

    dbg!(lines.next()); // light-to_temperature map:
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let n: Vec<_> = line
            .split_whitespace()
            .map(|n| str::parse::<u64>(n).unwrap())
            .collect();

        let dst_start = n[0];
        let src_start = n[1];
        let range_len = n[2];

        almanac
            .light_to_temperature
            .push((dst_start, src_start, range_len));
    }

    dbg!(lines.next()); // temperature-to-humidity map:
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let n: Vec<_> = line
            .split_whitespace()
            .map(|n| str::parse::<u64>(n).unwrap())
            .collect();

        let dst_start = n[0];
        let src_start = n[1];
        let range_len = n[2];

        almanac
            .temperature_to_humidity
            .push((dst_start, src_start, range_len));
    }

    dbg!(lines.next()); // humidity-to-location map:
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let n: Vec<_> = line
            .split_whitespace()
            .map(|n| str::parse::<u64>(n).unwrap())
            .collect();

        let dst_start = n[0];
        let src_start = n[1];
        let range_len = n[2];

        almanac
            .humidity_to_location
            .push((dst_start, src_start, range_len));
    }

    (seeds, almanac)
}

fn part1(input: &str) -> u64 {
    let (seeds, almanac) = parse_almanac(input);

    let mut locations = vec![];
    for seed in seeds {
        let mut val = seed;

        let maps = [
            &almanac.seed_to_soil,
            &almanac.soil_to_fertilizer,
            &almanac.fertilizer_to_water,
            &almanac.water_to_light,
            &almanac.light_to_temperature,
            &almanac.temperature_to_humidity,
            &almanac.humidity_to_location,
        ];

        for map in maps {
            for (dst_start, src_start, range_len) in map {
                if (*src_start..(*src_start + *range_len)).contains(&val) {
                    val = *dst_start + (val - *src_start);
                    break;
                }
            }
        }

        locations.push(val);
    }

    *locations.iter().min().unwrap()
}

fn part2(input: &str) -> u64 {
    let (seed_ranges, almanac) = parse_almanac(input);

    let seed_ranges: Vec<_> = seed_ranges
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    for location in 0.. {
        let maps = [
            &almanac.humidity_to_location,
            &almanac.temperature_to_humidity,
            &almanac.light_to_temperature,
            &almanac.water_to_light,
            &almanac.fertilizer_to_water,
            &almanac.soil_to_fertilizer,
            &almanac.seed_to_soil,
        ];

        let mut val = location;

        for map in maps {
            for (dst_start, src_start, range_len) in map {
                if (*dst_start..(*dst_start + *range_len)).contains(&val) {
                    val = *src_start + (val - *dst_start);
                    break;
                }
            }
        }

        for (range_start, range_len) in &seed_ranges {
            if (*range_start..(*range_start + *range_len)).contains(&val) {
                return location;
            }
        }
    }

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
    fn test_day05_part1_example() {
        let input = include_str!("../../inputs/day05_example.txt");
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 389056265);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day05_example.txt");
        assert_eq!(part2(input), 46);
    }

    #[ignore]
    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day05.txt");
        assert_eq!(part2(input), 137516820);
    }
}
