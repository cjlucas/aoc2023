const INPUT: &str = include_str!("../../inputs/day05.txt");

#[derive(Default, Debug)]
struct Map(Vec<(u64, u64, u64)>);

impl Map {
    fn src_to_dst(&self, src: u64) -> u64 {
        for (dst_start, src_start, range_len) in &self.0 {
            if (*src_start..(*src_start + *range_len)).contains(&src) {
                return *dst_start + (src - *src_start);
            }
        }

        src
    }

    fn dst_to_src(&self, dst: u64) -> u64 {
        for (dst_start, src_start, range_len) in &self.0 {
            if (*dst_start..(*dst_start + *range_len)).contains(&dst) {
                return *src_start + (dst - *dst_start);
            }
        }

        dst
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let mut val = seed;

        let maps = [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ];

        for map in maps {
            val = map.src_to_dst(val);
        }

        val
    }
}

fn parse_map(input: &str) -> Map {
    Map(input
        .lines()
        .skip(1)
        .map(|line| {
            let mut vals = line
                .split_whitespace()
                .map(|n| str::parse::<u64>(n).unwrap());

            (
                vals.next().unwrap(),
                vals.next().unwrap(),
                vals.next().unwrap(),
            )
        })
        .collect())
}

fn parse_almanac(input: &str) -> (Vec<u64>, Almanac) {
    let mut almanac = Almanac::default();

    let mut parts = input.split("\n\n");

    let (_, seeds) = parts.next().unwrap().split_once(':').unwrap();
    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .map(|seed| str::parse(seed).unwrap())
        .collect();

    almanac.seed_to_soil = parse_map(parts.next().unwrap());
    almanac.soil_to_fertilizer = parse_map(parts.next().unwrap());
    almanac.fertilizer_to_water = parse_map(parts.next().unwrap());
    almanac.water_to_light = parse_map(parts.next().unwrap());
    almanac.light_to_temperature = parse_map(parts.next().unwrap());
    almanac.temperature_to_humidity = parse_map(parts.next().unwrap());
    almanac.humidity_to_location = parse_map(parts.next().unwrap());

    (seeds, almanac)
}

fn part1(input: &str) -> u64 {
    let (seeds, almanac) = parse_almanac(input);

    seeds
        .into_iter()
        .map(|seed| almanac.map_seed_to_location(seed))
        .min()
        .unwrap()
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
            val = map.dst_to_src(val);
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
