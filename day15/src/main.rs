use array2d::Array2D;
use pathfinding::prelude::{absdiff, astar};

use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Array2D<u32>,
    col_max: usize,
    row_max: usize,
}

impl Grid {
    fn new(lines: &[Vec<u32>]) -> Grid {
        let grid = Array2D::from_rows(lines);
        let col_max = grid.num_columns() - 1;
        let row_max = grid.num_rows() - 1;
        Grid {
            grid,
            col_max,
            row_max,
        }
    }
    fn neighbors_with_cost(&self, vec: Vec<(usize, usize)>) -> Vec<(Pos, u32)> {
        vec.iter()
            .map(|p| (Pos(p.0, p.1), *self.grid.get(p.0, p.1).unwrap()))
            .collect()
    }
    fn successors(&self, p: &Pos) -> Vec<(Pos, u32)> {
        let &Pos(row, col) = p;
        self.neighbors_with_cost(match (row, col) {
            // upper left
            (0, 0) => vec![(0, 1), (1, 0)],
            // lower left
            (r, 0) if r == self.row_max => vec![(r - 1, 0), (r, 1)],
            // down the left side in the middle
            (r, 0) => vec![(r - 1, 0), (r, 1), (r + 1, 0)],
            // upper right
            (0, c) if c == self.col_max => vec![(0, c - 1), (1, c)],
            // Top row in the middle
            (0, c) => vec![(0, c - 1), (1, c), (0, c + 1)],
            // lower right
            (r, c) if r == self.row_max && c == self.col_max => vec![(r, c - 1), (r - 1, c)],
            // Bottom row in the middle
            (r, c) if r == self.row_max => vec![(r, c - 1), (r - 1, c), (r, c + 1)],
            // down the right side in the middle
            (r, c) if c == self.col_max => vec![(r - 1, c), (r, c - 1), (r + 1, c)],
            // Somewhere in the middle
            (r, c) => vec![(r - 1, c), (r, c - 1), (r, c + 1), (r + 1, c)],
        })
    }
}

fn main() {
    let lines: Vec<Vec<u32>> = BufReader::new(std::io::stdin())
        .lines()
        // Split each line into a Vec of chars
        .map(|l| l.unwrap().chars().collect::<_>())
        // Convert each char into a digit
        .map(|l: Vec<char>| {
            l.iter()
                .map(|c| c.to_digit(10_u32).unwrap())
                .collect::<Vec<u32>>()
        })
        .inspect(|x| println!("X {:?}", x))
        .collect();

    let mut extended_lines: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut tmp: Vec<u32> = Vec::new();
        for i in 0..5 {
            for item in line.clone() {
                let item = match item + i {
                    1..=9 => item + i,
                    10..=18 => item + i - 9,
                    19..=27 => item + i - 18,
                    _ => unreachable!(),
                };
                tmp.push(item);
            }
        }
        extended_lines.push(tmp);
    }

    println!("extended_lines:\n{:?}", extended_lines);

    //let shifts = (0..9)
    //    .map(|x| {
    //        lines
    //            .iter()
    //            .map(|l| l.iter().map(|i| match (i + x) {
    //                1..=9 => i+x,
    //                10..=18 => i+x-9,
    //                19..=27 => i+x-18,
    //                _ => unreachable!(),
    //            }).collect::<Vec<_>>())
    //            .collect::<Vec<_>>()
    //    })
    //    .inspect(|z| println!("Z: {:?}", z))
    //    .collect::<Vec<_>>();

    //println!("IT: {:?}", shifts);

    //let mut tmp: Vec<Vec<u32>> = Vec::new();
    //for i in 0..5 {
    //    for j in 0..5 {
    //    let mut inner: Vec<u32> = Vec::new();
    //        print!("{} ", j+i);
    //        for z in 0..i+5 {
    //            inner.extend(shifts[z][j+i].clone());
    //        }
    //    tmp.push(inner);
    //    }
    //    println!();
    //}
    //println!("COMBINED\n{:?}", tmp);
    let grid = Grid::new(&extended_lines);

    let start = Pos(0, 0);
    let goal: Pos = Pos(grid.row_max, grid.col_max);

    let (path, cost) = astar(
        &start,
        |p| grid.successors(p),
        |p| p.distance(&goal) / 3,
        |p| *p == goal,
    )
    .unwrap();

    println!("path: {:?}", path);
    println!("cost: {}", cost);
}
