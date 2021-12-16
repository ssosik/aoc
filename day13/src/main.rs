#![feature(iter_map_while)]

use array2d::Array2D;
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
    let mut pairs: Vec<(u32, u32)> = BufReader::new(std::io::stdin())
        .lines()
        .map_while(|line| {
            // Split each line on ',' character, passing the two pieces into the match
            match line.unwrap().split(',').collect::<Vec<&str>>()[..] {
                [a, b] => Some((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())),
                _ => None,
            }
        }).collect();
    println!("pairs {:?}", pairs);

    //let mut grid = Grid{
    //    grid:  Array2D::from_rows(
    //    &BufReader::new(std::io::stdin())
    //        .lines()
    //        .map(|line| {
    //            line.unwrap()
    //                .chars()
    //                //.inspect(|x| println!("{:?}", x))
    //                .map(|v| v.to_digit(10u32).unwrap() as usize)
    //                .collect::<Vec<_>>()
    //        })
    //        .collect::<Vec<_>>(),
    //), flash_cnt: 0};

    //let row_max = grid.row_max() as usize;
    //let col_max = grid.col_max() as usize;
    //println!("Before any steps:\n{}", grid);

}
