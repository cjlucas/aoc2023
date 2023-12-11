use aoc2023::prelude::*;

const INPUT: &str = include_str!("../../inputs/day10.txt");

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    // NW,
    N,
    // NE,
    W,
    E,
    // SW,
    S,
    // SE,
}

const DIRECTIONS: [Direction; 4] = [Direction::N, Direction::W, Direction::E, Direction::S];

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

struct Grid {
    nrows: usize,
    ncols: usize,
    tiles: HashMap<Point, char>,
}

impl Grid {
    fn adjacent_tile(&self, point: &Point, direction: Direction) -> Option<(Point, char)> {
        let (row_offset, col_offset): (i64, i64) = match direction {
            // Direction::NW => (-1, -1),
            Direction::N => (-1, 0),
            // Direction::NE => (-1, 1),
            Direction::W => (0, -1),
            Direction::E => (0, 1),
            // Direction::SW => (1, -1),
            Direction::S => (1, 0),
            // Direction::SE => (1, 1),
        };

        // handle negative coordinates
        if point.row == 0 && row_offset == -1 || point.col == 0 && col_offset == -1 {
            return None;
        }

        let row = ((point.row as i64) + row_offset) as usize;
        let col = ((point.col as i64) + col_offset) as usize;

        let adjacent_point = Point::new(row, col);

        self.tiles
            .get(&adjacent_point)
            .map(|c| (adjacent_point, *c))
    }

    fn next_direction(&self, dir: Direction, tile: char) -> Option<Direction> {
        match (dir, tile) {
            (Direction::N, '|') => Some(Direction::N),
            (Direction::S, '|') => Some(Direction::S),
            (Direction::E, '-') => Some(Direction::E),
            (Direction::W, '-') => Some(Direction::W),
            (Direction::S, 'L') => Some(Direction::E),
            (Direction::W, 'L') => Some(Direction::N),
            (Direction::E, 'J') => Some(Direction::N),
            (Direction::S, 'J') => Some(Direction::W),
            (Direction::E, '7') => Some(Direction::S),
            (Direction::N, '7') => Some(Direction::W),
            (Direction::N, 'F') => Some(Direction::E),
            (Direction::W, 'F') => Some(Direction::S),
            _ => None,
        }
    }
}

impl std::str::FromStr for Grid {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nrows = 0;
        let mut ncols = 0;
        let mut tiles = HashMap::new();

        for line in s.lines() {
            ncols = 0;

            for c in line.chars() {
                let point = Point::new(nrows, ncols);
                tiles.insert(point, c);
                ncols += 1;
            }

            nrows += 1;
        }

        Ok(Self {
            nrows,
            ncols,
            tiles,
        })
    }
}

fn part1(input: &str) -> usize {
    let grid: Grid = str::parse(input).unwrap();
    let start = grid.tiles.iter().find(|(_, v)| **v == 'S').unwrap().0;
    dbg!(start);

    let mut valid_starting_points = vec![];

    for dir in DIRECTIONS {
        dbg!(dir);
        let Some((p, tile)) = grid.adjacent_tile(start, dir) else {
            continue;
        };

        if grid.next_direction(dir, tile).is_some() {
            valid_starting_points.push((dir, p));
        }
    }

    dbg!(&valid_starting_points);

    let mut distances: HashMap<Point, Vec<usize>> = HashMap::new();

    for (mut cur_dir, starting_point) in valid_starting_points {
        let mut distance_from_start = 0;
        let mut cur_point = starting_point;

        loop {
            let cur_tile = grid.tiles.get(&cur_point).unwrap();
            if *cur_tile == 'S' {
                break;
            }

            distance_from_start += 1;
            distances
                .entry(cur_point)
                .or_default()
                .push(distance_from_start);

            cur_dir = grid.next_direction(cur_dir, *cur_tile).unwrap();
            cur_point = grid.adjacent_tile(&cur_point, cur_dir).unwrap().0;
        }
    }

    *distances
        .values()
        .map(|ds| ds.into_iter().min().unwrap())
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let grid: Grid = str::parse(input).unwrap();

    for row in 0..grid.nrows {
        for col in 0..grid.ncols {
            let p = Point::new(row, col);
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
    fn test_part1_example1() {
        let input = include_str!("../../inputs/day10_example1.txt");
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part1_example2() {
        let input = include_str!("../../inputs/day10_example2.txt");
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6870);
    }

    #[test]
    fn test_part2_example1() {
        let input = include_str!("../../inputs/day10p2_example1.txt");
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn test_part2_example2() {
        let input = include_str!("../../inputs/day10p2_example2.txt");
        assert_eq!(part2(input), 8);
    }

    #[test]
    fn test_part2_example3() {
        let input = include_str!("../../inputs/day10p2_example3.txt");
        assert_eq!(part2(input), 10);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), -1);
    // }
}
