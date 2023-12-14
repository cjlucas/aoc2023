use aoc2023::prelude::*;

const INPUT: &str = include_str!("../../inputs/day11.txt");

// struct Point {
//     row: usize,
//     col: usize,
// }

struct Grid {
    // nrows: usize,
    // ncols: usize,
    tiles: Vec<Vec<char>>,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        let num_rows = self.tiles.len();

        for (idx, row) in self.tiles.iter().enumerate() {
            for c in row {
                f.write_char(*c)?;
            }

            if idx < num_rows - 1 {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

impl std::str::FromStr for Grid {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();

        for line in s.lines() {
            let row: Vec<_> = line.chars().collect();

            // if row.iter().all(|c| *c == '.') {
            //     tiles.push(row.clone());
            // }

            tiles.push(row);
        }

        Ok(Grid { tiles })
    }
}

impl Grid {
    fn expand(&mut self, expansion_rate: usize) {
        let mut rows = vec![];
        let mut cols = vec![];

        for (num_row, row) in self.tiles.iter().enumerate() {
            if row.iter().all(|c| *c == '.') {
                rows.push(num_row + (rows.len() * expansion_rate))
            }
        }

        for num_col in 0..self.tiles.first().unwrap().len() {
            let mut col = self.tiles.iter().map(|row| row[num_col]);
            if col.all(|c| c == '.') {
                cols.push(num_col + (cols.len() * expansion_rate))
            }
        }

        let col = vec!['.'; self.tiles.first().unwrap().len()];

        for num_row in rows {
            for _ in 0..expansion_rate {
                self.tiles.insert(num_row, col.clone());
            }
        }

        for num_col in cols {
            for row in &mut self.tiles {
                for _ in 0..expansion_rate {
                    row.insert(num_col, '.');
                }
            }
        }
    }
}

fn solve(input: &str, expansion_rate: usize) -> i64 {
    let mut grid: Grid = str::parse(input).unwrap();
    println!("expanding with rate: {}", expansion_rate);
    grid.expand(expansion_rate);
    println!("done expanding");

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (num_row, row) in grid.tiles.iter().enumerate() {
        for (num_col, ch) in row.iter().enumerate() {
            if *ch == '#' {
                galaxies.push((num_col, num_row));
            }
        }
    }

    let mut i = 0;
    let mut sums = vec![];

    for (a, b) in iproduct!(0..galaxies.len(), 0..galaxies.len()) {
        if a >= b {
            continue;
        }

        let gala = galaxies[a];
        let galb = galaxies[b];

        if a == 0 && b == 6 {
            println!("{:?}, {:?}", gala, galb);
        }

        sums.push(gala.0.abs_diff(galb.0) as i64 + gala.1.abs_diff(galb.1) as i64);

        i += 1;
    }

    dbg!(galaxies.len());
    dbg!(i);

    sums.into_iter().sum()
}

fn part1(input: &str) -> i64 {
    solve(input, 2 - 1)
}

fn part2(input: &str) -> i64 {
    solve(input, 1_000_000 - 1)
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod grid {
        use super::*;

        #[test]
        fn test_grid_display() {
            let input = include_str!("../../inputs/day11_example.txt").trim();
            let grid: Grid = str::parse(input).unwrap();

            assert_eq!(format!("{}", grid), input);
        }

        #[test]
        fn test_grid_expand() {
            let input = include_str!("../../inputs/day11_example.txt").trim();
            let mut grid: Grid = str::parse(input).unwrap();
            grid.expand(1);

            let expected = include_str!("../../inputs/day11_example_expanded.txt").trim();
            assert_eq!(format!("{}", grid), expected);
        }
    }

    #[test]
    fn test_part1_example() {
        let input = include_str!("../../inputs/day11_example.txt");
        assert_eq!(part1(input), 374);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 10228230);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../../inputs/day11_example.txt");
        assert_eq!(solve(input, 9), 1030);
        assert_eq!(solve(input, 99), 8410);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), -1);
    // }
}
