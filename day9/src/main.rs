use array2d::Array2D;
use std::collections::BTreeSet;
use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    println!("Hello, world!");
    let heightmap = Array2D::from_rows(
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
    );
    let mut low_points: Vec<u32> = Vec::new();
    let row_max = heightmap.num_rows() - 1;
    let col_max = heightmap.num_columns() - 1;
    println!("HeightMap {:?} {} {}", heightmap, row_max, col_max);
    for (r, row) in heightmap.rows_iter().enumerate() {
        for (c, item) in row.enumerate() {
            //print!("r{} c{} {:>2}  ", r, c, item);
            let mut neighbors: BTreeSet<&u32> = BTreeSet::new();
            match (r, c) {
                // Anywhere in the middle
                (y, x) if 0 < x && x < col_max && 0 < y && y < row_max => {
                    neighbors.insert(heightmap.get(r - 1, c).unwrap());
                    neighbors.insert(heightmap.get(r + 1, c).unwrap());
                    neighbors.insert(heightmap.get(r, c - 1).unwrap());
                    neighbors.insert(heightmap.get(r, c + 1).unwrap());
                }
                // Along the top row excluding the corners
                (y, x) if 0 < x && x < col_max && y == 0 => {
                    neighbors.insert(heightmap.get(r + 1, c).unwrap());
                    neighbors.insert(heightmap.get(r, c - 1).unwrap());
                    neighbors.insert(heightmap.get(r, c + 1).unwrap());
                }
                // Along the left column excluding the corners
                (y, x) if x == 0 && 0 < y && y < row_max => {
                    neighbors.insert(heightmap.get(r - 1, c).unwrap());
                    neighbors.insert(heightmap.get(r + 1, c).unwrap());
                    neighbors.insert(heightmap.get(r, c + 1).unwrap());
                }
                // Along the bottom row excluding the corners
                (y, x) if 0 < x && x < col_max && y == row_max => {
                    neighbors.insert(heightmap.get(r - 1, c).unwrap());
                    neighbors.insert(heightmap.get(r, c - 1).unwrap());
                    neighbors.insert(heightmap.get(r, c + 1).unwrap());
                }
                // Along the right column excluding the corners
                (y, x) if x == col_max && 0 < y && y < row_max => {
                    neighbors.insert(heightmap.get(r - 1, c).unwrap());
                    neighbors.insert(heightmap.get(r + 1, c).unwrap());
                    neighbors.insert(heightmap.get(r, c - 1).unwrap());
                }
                // Top Left corner
                (y, x) if x == 0 && y == 0 => {
                    neighbors.insert(heightmap.get(r, c + 1).unwrap());
                    neighbors.insert(heightmap.get(r + 1, c).unwrap());
                }
                // Top Right corner
                (y, x) if x == col_max && y == 0 => {
                    neighbors.insert(heightmap.get(r, c - 1).unwrap());
                    neighbors.insert(heightmap.get(r + 1, c).unwrap());
                }
                // Bottom Left corner
                (y, x) if x == 0 && y == row_max => {
                    neighbors.insert(heightmap.get(r, c + 1).unwrap());
                    neighbors.insert(heightmap.get(r - 1, c).unwrap());
                }
                // Bottom Right corner
                (y, x) if x == col_max && y == row_max => {
                    neighbors.insert(heightmap.get(r, c - 1).unwrap());
                    neighbors.insert(heightmap.get(r - 1, c).unwrap());
                }
                _ => unreachable!(),
            };
            //println!("  neighbors {:?}", neighbors);
            if neighbors
                .iter().find(|&x| *x <= item).is_none()
            {
                low_points.push(*item);
            }
        }
    }
    println!("low_points {:?} {}", low_points, low_points.iter().map(|x| x + 1).sum::<u32>());

    Ok(())
}
