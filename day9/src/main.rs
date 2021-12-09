use array2d::Array2D;
use std::collections::BTreeSet;
use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct Grid(Array2D<u32>);

impl Grid {
    fn row_max(&self) -> u32 {
        self.0.num_rows() as u32 - 1
    }

    fn col_max(&self) -> u32 {
        self.0.num_columns() as u32 - 1
    }

    fn get_neighbor_coords(&self, r: u32, c: u32) -> Result<BTreeSet<(u32, u32)>> {
        let mut neighbors: BTreeSet<(u32, u32)> = BTreeSet::new();

        let row_max = self.row_max();
        let col_max = self.col_max();
        //println!("HeightMap {:?} {} {}", self, row_max, col_max);

        match (r, c) {
            // Anywhere in the middle
            (y, x) if 0 < x && x < col_max && 0 < y && y < row_max => {
                neighbors.insert((r - 1, c));
                neighbors.insert((r + 1, c));
                neighbors.insert((r, c - 1));
                neighbors.insert((r, c + 1));
            }
            // Along the top row excluding the corners
            (y, x) if 0 < x && x < col_max && y == 0 => {
                neighbors.insert((r + 1, c));
                neighbors.insert((r, c - 1));
                neighbors.insert((r, c + 1));
            }
            // Along the left column excluding the corners
            (y, x) if x == 0 && 0 < y && y < row_max => {
                neighbors.insert((r - 1, c));
                neighbors.insert((r + 1, c));
                neighbors.insert((r, c + 1));
            }
            // Along the bottom row excluding the corners
            (y, x) if 0 < x && x < col_max && y == row_max => {
                neighbors.insert((r - 1, c));
                neighbors.insert((r, c - 1));
                neighbors.insert((r, c + 1));
            }
            // Along the right column excluding the corners
            (y, x) if x == col_max && 0 < y && y < row_max => {
                neighbors.insert((r - 1, c));
                neighbors.insert((r + 1, c));
                neighbors.insert((r, c - 1));
            }
            // Top Left corner
            (y, x) if x == 0 && y == 0 => {
                neighbors.insert((r, c + 1));
                neighbors.insert((r + 1, c));
            }
            // Top Right corner
            (y, x) if x == col_max && y == 0 => {
                neighbors.insert((r, c - 1));
                neighbors.insert((r + 1, c));
            }
            // Bottom Left corner
            (y, x) if x == 0 && y == row_max => {
                neighbors.insert((r, c + 1));
                neighbors.insert((r - 1, c));
            }
            // Bottom Right corner
            (y, x) if x == col_max && y == row_max => {
                neighbors.insert((r, c - 1));
                neighbors.insert((r - 1, c));
            }
            _ => unreachable!(),
        };
        Ok(neighbors)
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let heightmap = Grid(Array2D::from_rows(
        &BufReader::new(std::io::stdin())
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    //.inspect(|x| println!("{:?}", x))
                    .filter_map(|v| v.to_digit(10u32))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    ));
    let mut low_points: Vec<u32> = Vec::new();
    for (r, row) in heightmap.0.rows_iter().enumerate() {
        for (c, item) in row.enumerate() {
            if !heightmap
                .get_neighbor_coords(r as u32, c as u32)?
                .iter()
                .any(|(x, y)| heightmap.0.get(*x as usize, *y as usize).unwrap() <= item)
            {
                low_points.push(*item);
            }
        }
    }
    println!(
        "low_points {:?} {}",
        low_points,
        low_points.iter().map(|x| x + 1).sum::<u32>()
    );

    Ok(())
}
