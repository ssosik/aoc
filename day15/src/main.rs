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
struct Grid(Array2D<u32>);

impl Grid {
    fn new(lines: &[Vec<u32>]) -> Grid {
        Grid(Array2D::from_rows(lines))
    }
    fn neighbors_with_cost(&self, vec: Vec<(usize, usize)>) -> Vec<(Pos, u32)> {
        vec.iter()
            .map(|p| (Pos(p.0, p.1), *self.0.get(p.0, p.1).unwrap()))
            .collect()
    }
    fn successors(&self, p: &Pos) -> Vec<(Pos, u32)> {
        let &Pos(row, col) = p;
        let col_max = self.0.num_columns() - 1;
        let row_max = self.0.num_rows() - 1;
        //println!("HERE {} {}", row, col);
        match (row, col) {
            // upper left
            (0, 0) => self.neighbors_with_cost(vec![(0, 1), (1, 0)]),
            // lower left
            (r, 0) if r == row_max => self.neighbors_with_cost(vec![(r - 1, 0), (r, 1)]),
            // down the left side in the middle
            (r, 0) => self.neighbors_with_cost(vec![(r - 1, 0), (r, 1), (r + 1, 0)]),
            // upper right
            (0, c) if c == col_max => self.neighbors_with_cost(vec![(0, c - 1), (1, c)]),
            // Top row in the middle
            (0, c) => self.neighbors_with_cost(vec![(0, c - 1), (1, c), (0, c + 1)]),
            // lower right
            (r, c) if r == row_max && c == col_max => {
                self.neighbors_with_cost(vec![(r, c - 1), (r - 1, c)])
            }
            // Bottom row in the middle
            (r, c) if r == row_max => {
                self.neighbors_with_cost(vec![(r, c - 1), (r - 1, c), (r, c + 1)])
            }
            // down the right side in the middle
            (r, c) if c == col_max => {
                self.neighbors_with_cost(vec![(r - 1, c), (r, c - 1), (r + 1, c)])
            }
            // Somewhere in the middle
            (r, c) => {
                self.neighbors_with_cost(vec![(r - 1, c), (r, c - 1), (r, c + 1), (r + 1, c)])
            }
        }
    }
}

fn main() {
    let lines: Vec<Vec<u32>> = BufReader::new(std::io::stdin())
        .lines()
        //.map(|l| l.unwrap().chars().map(|c| c.to_digit(10_u32).unwrap()))
        .map(|l| l.unwrap().chars().collect::<_>())
        .map(|l: Vec<char>| {
            l.iter()
                .map(|c| c.to_digit(10_u32).unwrap())
                .collect::<Vec<u32>>()
        })
        .inspect(|x| println!("X {:?}", x))
        .collect();
    let grid = Grid::new(&lines);
    let rows = grid.0.num_rows();
    let columns = grid.0.num_columns();

    let start = Pos(0, 0);
    let goal: Pos = Pos(rows - 1, columns - 1);

    println!("lines: {:?}", lines);
    println!("rows: {} columns {}", rows, columns);
    println!("goal {}", grid.0.get(rows - 1, columns - 1).unwrap());

    let result = astar(
        &start,
        |p| grid.successors(p),
        |p| p.distance(&goal) / 3,
        |p| *p == goal,
    );
    println!("result: {:?}", result);
}
