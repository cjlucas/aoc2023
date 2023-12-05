use aoc2023::prelude::*;

use std::collections::{btree_set::Difference, HashMap};

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

fn part1(input: &str) -> u64 {
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

    let mut locations = vec![];
    for seed in seeds {
        let mut soil = None;
        for (dst_start, src_start, range_len) in &almanac.seed_to_soil {
            if (*src_start..(*src_start + *range_len)).contains(&seed) {
                soil = Some(*dst_start + (seed - *src_start));
            }
        }

        if soil.is_none() {
            soil = Some(seed);
        }

        let soil = soil.unwrap();

        let mut fertilizer = None;
        for (dst_start, src_start, range_len) in &almanac.soil_to_fertilizer {
            if (*src_start..(*src_start + *range_len)).contains(&soil) {
                fertilizer = Some(*dst_start + (soil - *src_start));
            }
        }

        if fertilizer.is_none() {
            fertilizer = Some(soil);
        }

        let fertilizer = fertilizer.unwrap();

        let mut water = None;
        for (dst_start, src_start, range_len) in &almanac.fertilizer_to_water {
            if (*src_start..(*src_start + *range_len)).contains(&fertilizer) {
                water = Some(*dst_start + (fertilizer - *src_start));
            }
        }

        if water.is_none() {
            water = Some(fertilizer);
        }

        let water = water.unwrap();

        let mut light = None;
        for (dst_start, src_start, range_len) in &almanac.water_to_light {
            if (*src_start..(*src_start + *range_len)).contains(&water) {
                light = Some(*dst_start + (water - *src_start));
            }
        }

        if light.is_none() {
            light = Some(water);
        }

        let light = light.unwrap();

        let mut temperature = None;
        for (dst_start, src_start, range_len) in &almanac.light_to_temperature {
            if (*src_start..(*src_start + *range_len)).contains(&light) {
                temperature = Some(*dst_start + (light - *src_start));
            }
        }

        if temperature.is_none() {
            temperature = Some(light);
        }

        let temperature = temperature.unwrap();

        let mut humidity = None;
        for (dst_start, src_start, range_len) in &almanac.temperature_to_humidity {
            if (*src_start..(*src_start + *range_len)).contains(&temperature) {
                humidity = Some(*dst_start + (temperature - *src_start));
            }
        }

        if humidity.is_none() {
            humidity = Some(temperature);
        }

        let humidity = humidity.unwrap();

        let mut location = None;
        for (dst_start, src_start, range_len) in &almanac.humidity_to_location {
            if (*src_start..(*src_start + *range_len)).contains(&humidity) {
                location = Some(*dst_start + (humidity - *src_start));
            }
        }

        if location.is_none() {
            location = Some(humidity);
        }

        let location = location.unwrap();

        // println!(
        //     "seed = {seed}, soil = {soil}, fert = {fertilizer}, water = {water}, light = {light}, temp = {temperature}, humidity = {humidity}"
        // );
        // , water = {water}, light = {light}, temperature = {temperature}, humidity = {humidity}, location = {location}");

        locations.push(location);
    }

    // dbg!(&almanac);

    *locations.iter().min().unwrap()
}

fn part2(input: &str) -> u64 {
    let mut almanac = Almanac::default();

    let mut lines = input.lines();
    let (_, seed_ranges) = lines.next().unwrap().split_once(':').unwrap();
    let seed_ranges: Vec<_> = seed_ranges
        .split_whitespace()
        .map(|seed| str::parse::<u64>(seed).unwrap())
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
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

    let mut locations = vec![];
    for (range_start, num_seeds) in seed_ranges {
        for i in 0..num_seeds {
            let seed = range_start + i;

            let mut soil = None;
            for (dst_start, src_start, range_len) in &almanac.seed_to_soil {
                if (*src_start..(*src_start + *range_len)).contains(&seed) {
                    soil = Some(*dst_start + (seed - *src_start));
                }
            }

            if soil.is_none() {
                soil = Some(seed);
            }

            let soil = soil.unwrap();

            let mut fertilizer = None;
            for (dst_start, src_start, range_len) in &almanac.soil_to_fertilizer {
                if (*src_start..(*src_start + *range_len)).contains(&soil) {
                    fertilizer = Some(*dst_start + (soil - *src_start));
                }
            }

            if fertilizer.is_none() {
                fertilizer = Some(soil);
            }

            let fertilizer = fertilizer.unwrap();

            let mut water = None;
            for (dst_start, src_start, range_len) in &almanac.fertilizer_to_water {
                if (*src_start..(*src_start + *range_len)).contains(&fertilizer) {
                    water = Some(*dst_start + (fertilizer - *src_start));
                }
            }

            if water.is_none() {
                water = Some(fertilizer);
            }

            let water = water.unwrap();

            let mut light = None;
            for (dst_start, src_start, range_len) in &almanac.water_to_light {
                if (*src_start..(*src_start + *range_len)).contains(&water) {
                    light = Some(*dst_start + (water - *src_start));
                }
            }

            if light.is_none() {
                light = Some(water);
            }

            let light = light.unwrap();

            let mut temperature = None;
            for (dst_start, src_start, range_len) in &almanac.light_to_temperature {
                if (*src_start..(*src_start + *range_len)).contains(&light) {
                    temperature = Some(*dst_start + (light - *src_start));
                }
            }

            if temperature.is_none() {
                temperature = Some(light);
            }

            let temperature = temperature.unwrap();

            let mut humidity = None;
            for (dst_start, src_start, range_len) in &almanac.temperature_to_humidity {
                if (*src_start..(*src_start + *range_len)).contains(&temperature) {
                    humidity = Some(*dst_start + (temperature - *src_start));
                }
            }

            if humidity.is_none() {
                humidity = Some(temperature);
            }

            let humidity = humidity.unwrap();

            let mut location = None;
            for (dst_start, src_start, range_len) in &almanac.humidity_to_location {
                if (*src_start..(*src_start + *range_len)).contains(&humidity) {
                    location = Some(*dst_start + (humidity - *src_start));
                }
            }

            if location.is_none() {
                location = Some(humidity);
            }

            let location = location.unwrap();

            // println!(
            //     "seed = {seed}, soil = {soil}, fert = {fertilizer}, water = {water}, light = {light}, temp = {temperature}, humidity = {humidity}"
            // );
            // , water = {water}, light = {light}, temperature = {temperature}, humidity = {humidity}, location = {location}");

            locations.push(location);
            dbg!(locations.len());
        }

        // dbg!(&almanac);
    }

    *locations.iter().min().unwrap()
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

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("../../inputs/day05.txt");
    //     assert_eq!(part2(input), 71585);
    // }
}
