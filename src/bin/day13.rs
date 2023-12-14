use aoc2023::prelude::*;

const INPUT: &str = include_str!("../../inputs/day13.txt");

fn vec_diff(a: &Vec<char>, b: &Vec<char>) -> usize {
    a.iter().zip(b).filter(|(a, b)| a != b).count()
}

#[derive(Debug)]
struct Row<'a> {
    idx: usize,
    grid: &'a Grid,
}

impl<'a> Row<'a> {
    fn chars(&self) -> Vec<char> {
        (0..self.grid.ncols)
            .map(|col_idx| {
                let p = Point {
                    row: self.idx,
                    col: col_idx,
                };
                *self.grid.tiles.get(&p).unwrap()
            })
            .collect()
    }
}

struct Rows<'a> {
    cur_idx: usize,
    rev_cur_idx: usize,
    grid: &'a Grid,
}

impl<'a> Iterator for Rows<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx > self.rev_cur_idx {
            return None;
        }

        if self.cur_idx < self.grid.nrows {
            let row = Some(Row {
                idx: self.cur_idx,
                grid: self.grid,
            });
            self.cur_idx += 1;
            row
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.grid.nrows - self.cur_idx;
        (n, Some(n))
    }
}

#[derive(Debug)]
struct Column<'a> {
    idx: usize,
    grid: &'a Grid,
}

impl<'a> Column<'a> {
    fn chars(&self) -> Vec<char> {
        (0..self.grid.nrows)
            .map(|row_idx| {
                let p = Point {
                    row: row_idx,
                    col: self.idx,
                };

                // dbg!(&p);
                *self.grid.tiles.get(&p).unwrap()
            })
            .collect()
    }
}

struct Columns<'a> {
    cur_idx: usize,
    grid: &'a Grid,
}

impl<'a> Iterator for Columns<'a> {
    type Item = Column<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_idx < self.grid.ncols {
            let col = Some(Column {
                idx: self.cur_idx,
                grid: self.grid,
            });
            self.cur_idx += 1;
            col
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for Columns<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.cur_idx > 0 {
            self.cur_idx -= 1;
            Some(Column {
                idx: self.cur_idx,
                grid: self.grid,
            })
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for Columns<'a> {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Point {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Grid {
    nrows: usize,
    ncols: usize,
    tiles: HashMap<Point, char>,
}

impl Grid {
    fn rows(&self) -> Rows {
        Rows {
            cur_idx: 0,
            rev_cur_idx: self.nrows,
            grid: self,
        }
    }

    fn cols(&self) -> Columns {
        Columns {
            cur_idx: 0,
            grid: self,
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

            for ch in line.chars() {
                let p = Point {
                    row: nrows,
                    col: ncols,
                };

                tiles.insert(p, ch);
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

fn solve(grid: &Grid) -> i64 {
    // println!("lookin at rows..");

    let mut rows = grid.rows().enumerate().peekable();
    while let Some((idx, row)) = rows.next() {
        // println!("{idx} {:?}", row.chars());

        let Some((_, rowb)) = rows.peek() else {
            break;
        };

        if row.chars() == rowb.chars() {
            // println!(
            //     "found a match {} ({:?} == {:?})",
            //     idx,
            //     row.chars(),
            //     rowb.chars()
            // );

            // dbg!(idx);

            // let foo: Vec<_> = grid
            //     .rows()
            //     .take(idx + 1)
            //     .collect::<Vec<_>>()
            //     .into_iter()
            //     .rev()
            //     .map(|row| row.chars())
            //     .collect();
            // println!("{:?}", foo);
            // let foo: Vec<_> = grid.rows().skip(idx).map(|row| row.chars()).collect();
            // println!("{:?}", foo);

            if grid
                .rows()
                .take(idx + 1)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .zip(grid.rows().skip(idx + 1))
                .all(|(a, b)| a.chars() == b.chars())
            {
                // println!("found da line {idx}");
                return (100 * (idx + 1)) as i64;
            }
        }
    }

    // println!("lookin at cols..");

    let mut cols = grid.cols().enumerate().peekable();
    while let Some((idx, col)) = cols.next() {
        // println!("{idx} {:?}", col.chars());

        let Some((_, colb)) = cols.peek() else {
            break;
        };

        if col.chars() == colb.chars() {
            // println!(
            //     "found a match {} ({:?} == {:?})",
            //     idx,
            //     col.chars(),
            //     colb.chars()
            // );

            // dbg!(idx);

            // let foo: Vec<_> = grid
            //     .rows()
            //     .take(idx + 1)
            //     .collect::<Vec<_>>()
            //     .into_iter()
            //     .rev()
            //     .map(|row| row.chars())
            //     .collect();
            // println!("{:?}", foo);
            // let foo: Vec<_> = grid.rows().skip(idx).map(|row| row.chars()).collect();
            // println!("{:?}", foo);

            if grid
                .cols()
                .take(idx + 1)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .zip(grid.cols().skip(idx + 1))
                .all(|(a, b)| a.chars() == b.chars())
            {
                // println!("found da line {idx}");
                return (idx + 1) as i64;
            }
        }
    }

    0
}

fn part1(input: &str) -> i64 {
    let grids: Vec<_> = input
        .split("\n\n")
        .map(|grid| str::parse::<Grid>(grid).unwrap())
        .collect();

    let mut sum = 0;

    for grid in grids {
        sum += solve(&grid);
    }

    sum as i64
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
        let input = include_str!("../../inputs/day13_example.txt");
        assert_eq!(part1(input), 405);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 41859);
    }

    // #[test]
    // fn test_part2_example() {
    //     let input = include_str!("../../inputs/day13_example.txt");
    //     assert_eq!(part2(input), -1);
    // }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), -1);
    // }
}
