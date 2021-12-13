use array2d::Array2D;
use std::collections::BTreeSet;
use std::error;
use std::fmt;
use std::io::{BufRead, BufReader};

type MyResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct Grid{
    grid: Array2D<usize>,
    flash_cnt: usize,
}

impl Grid {
    fn row_max(&self) -> usize {
        self.grid.num_rows() as usize - 1
    }

    fn col_max(&self) -> usize {
        self.grid.num_columns() as usize - 1
    }

    fn get(&self, r: usize, c: usize) -> usize {
        *self.grid.get(r, c).unwrap()
    }

    fn propagate_flash(&mut self, r: usize, c: usize) {
        if self.grid.get(r, c).unwrap() > &9 {
            self.flash_cnt += 1;
            *self.grid.get_mut(r, c).unwrap() = 0;
            for nbr in self.get_neighbors(r, c).unwrap() {
                if let Some(x) = self.grid.get_mut(nbr.0, nbr.1) {
                    if *x == 0 {
                        // This node coordinate has already been processed
                        continue;
                    }
                    if *x <= 9 {
                        // Increment the neighbor coordinate
                        *x += 1;
                    }
                    if *x > 9 {
                        // recursively propagate the flash
                        self.propagate_flash(nbr.0, nbr.1);
                    }
                }
            }
        }
    }

    fn get_neighbors(&self, r: usize, c: usize) -> MyResult<BTreeSet<(usize, usize, usize)>> {
        let mut neighbors: BTreeSet<(usize, usize, usize)> = BTreeSet::new();

        let row_max = self.row_max();
        let col_max = self.col_max();

        match (r, c) {
            // Anywhere in the middle
            (y, x) if 0 < x && x < col_max && 0 < y && y < row_max => {
                neighbors.insert((r - 1, c, self.get(r - 1, c)));
                neighbors.insert((r + 1, c, self.get(r + 1, c)));
                neighbors.insert((r, c - 1, self.get(r, c - 1)));
                neighbors.insert((r, c + 1, self.get(r, c + 1)));
                neighbors.insert((r - 1, c - 1, self.get(r - 1, c - 1)));
                neighbors.insert((r - 1, c + 1, self.get(r - 1, c + 1)));
                neighbors.insert((r + 1, c - 1, self.get(r + 1, c - 1)));
                neighbors.insert((r + 1, c + 1, self.get(r + 1, c + 1)));
            }
            // Along the top row excluding the corners
            (y, x) if 0 < x && x < col_max && y == 0 => {
                neighbors.insert((r + 1, c, self.get(r + 1, c)));
                neighbors.insert((r + 1, c - 1, self.get(r + 1, c - 1)));
                neighbors.insert((r + 1, c + 1, self.get(r + 1, c + 1)));
                neighbors.insert((r, c - 1, self.get(r, c - 1)));
                neighbors.insert((r, c + 1, self.get(r, c + 1)));
            }
            // Along the left column excluding the corners
            (y, x) if x == 0 && 0 < y && y < row_max => {
                neighbors.insert((r - 1, c, self.get(r - 1, c)));
                neighbors.insert((r - 1, c + 1, self.get(r - 1, c + 1)));
                neighbors.insert((r + 1, c, self.get(r + 1, c)));
                neighbors.insert((r + 1, c + 1, self.get(r + 1, c + 1)));
                neighbors.insert((r, c + 1, self.get(r, c + 1)));
            }
            // Along the bottom row excluding the corners
            (y, x) if 0 < x && x < col_max && y == row_max => {
                neighbors.insert((r - 1, c, self.get(r - 1, c)));
                neighbors.insert((r - 1, c - 1, self.get(r - 1, c - 1)));
                neighbors.insert((r - 1, c + 1, self.get(r - 1, c + 1)));
                neighbors.insert((r, c - 1, self.get(r, c - 1)));
                neighbors.insert((r, c + 1, self.get(r, c + 1)));
            }
            // Along the right column excluding the corners
            (y, x) if x == col_max && 0 < y && y < row_max => {
                neighbors.insert((r - 1, c, self.get(r - 1, c)));
                neighbors.insert((r - 1, c - 1, self.get(r - 1, c - 1)));
                neighbors.insert((r + 1, c, self.get(r + 1, c)));
                neighbors.insert((r + 1, c - 1, self.get(r + 1, c - 1)));
                neighbors.insert((r, c - 1, self.get(r, c - 1)));
            }
            // Top Left corner
            (y, x) if x == 0 && y == 0 => {
                neighbors.insert((r, c + 1, self.get(r, c + 1)));
                neighbors.insert((r + 1, c, self.get(r + 1, c)));
                neighbors.insert((r + 1, c + 1, self.get(r + 1, c + 1)));
            }
            // Top Right corner
            (y, x) if x == col_max && y == 0 => {
                neighbors.insert((r, c - 1, self.get(r, c - 1)));
                neighbors.insert((r + 1, c, self.get(r + 1, c)));
                neighbors.insert((r + 1, c - 1, self.get(r + 1, c - 1)));
            }
            // Bottom Left corner
            (y, x) if x == 0 && y == row_max => {
                neighbors.insert((r, c + 1, self.get(r, c + 1)));
                neighbors.insert((r - 1, c, self.get(r - 1, c)));
                neighbors.insert((r - 1, c + 1, self.get(r - 1, c + 1)));
            }
            // Bottom Right corner
            (y, x) if x == col_max && y == row_max => {
                neighbors.insert((r, c - 1, self.get(r, c - 1)));
                neighbors.insert((r - 1, c, self.get(r - 1, c)));
                neighbors.insert((r - 1, c - 1, self.get(r - 1, c - 1)));
            }
            _ => unreachable!(),
        };
        Ok(neighbors)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.grid.rows_iter() {
            for item in row {
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    // Load the initial input
    let mut grid = Grid{
        grid:  Array2D::from_rows(
        &BufReader::new(std::io::stdin())
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    //.inspect(|x| println!("{:?}", x))
                    .map(|v| v.to_digit(10u32).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    ), flash_cnt: 0};

    let row_max = grid.row_max() as usize;
    let col_max = grid.col_max() as usize;
    println!("Before any steps:\n{}", grid);

    for step in 0..100 {
        // Perform a step increment
        for row in 0..=row_max {
            for col in 0..=col_max {
                if let Some(x) = grid.grid.get_mut(row, col) {
                    *x += 1
                }
            }
        }

        // Look for any octopuses that need to flash. If any
        // are found, propagate the increments to its
        // neighbors
        for row in 0..=row_max {
            for col in 0..=col_max {
                grid.propagate_flash(row, col);
            }
        }

        println!("Step {} grid; flash count {}\n{}", step + 1, grid.flash_cnt, grid);
    }
}
