#![feature(iter_map_while)]

use array2d::Array2D;
use std::cmp;
use std::error;
use std::fmt;
use std::io::{BufRead, BufReader};

type MyResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct Grid(Array2D<bool>);

impl Grid {
    fn new(rows: usize, cols: usize) -> Grid {
        Grid(Array2D::filled_with(false, rows, cols))
    }
    fn fold(&self, fold: Fold) -> Grid {
        match fold {
            Fold::Horizontal(n) => Grid(Array2D::from_rows(
                &self.0.as_rows()[0..n]
                    .iter()
                    .zip(self.0.as_rows()[n..].iter().rev())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b)
                            .map(|(x, y)| *x || *y)
                            .collect::<Vec<bool>>()
                    })
                    .collect::<Vec<Vec<bool>>>(),
            )),
            Fold::Vertical(n) => Grid(Array2D::from_columns(
                &self.0.as_columns()[0..n]
                    .iter()
                    .zip(self.0.as_columns()[n..].iter().rev())
                    .map(|(a, b)| {
                        a.iter()
                            .zip(b)
                            .map(|(x, y)| *x || *y)
                            .collect::<Vec<bool>>()
                    })
                    .collect::<Vec<Vec<bool>>>(),
            )),
        }
    }
    fn set(&mut self, r: usize, c: usize, v: bool) -> Result<(), array2d::Error> {
        self.0.set(r, c, v)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in self.0.rows_iter() {
            for item in row {
                let item = if *item { '#' } else { '.' };
                write!(f, "{}", item)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

enum Fold {
    Horizontal(usize),
    Vertical(usize),
}
use std::str::FromStr;
impl FromStr for Fold {
    //type Err = Box<dyn std::error::Error>;
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // fold along y=7
        if s.starts_with("fold along") {
            // Skip the first 2 words, taking the 3rd 'y=7'
            let item: String = s.split_whitespace().skip(2).take(1).collect();
            // Split on '=' character and parse the values
            match item.split('=').collect::<Vec<&str>>()[..] {
                [a, b] => match a {
                    "y" => return Ok(Fold::Horizontal(b.parse::<usize>().unwrap())),
                    "x" => return Ok(Fold::Vertical(b.parse::<usize>().unwrap())),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        Err("Nope")
        //unreachable!("Should not be here {}", s)
    }
}
impl fmt::Display for Fold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fold::Horizontal(n) => write!(f, "horizontal {}", n)?,
            Fold::Vertical(n) => write!(f, "vertical {}", n)?,
        }
        Ok(())
    }
}

fn main() {
    let lines: Vec<_> = BufReader::new(std::io::stdin()).lines().collect();

    // Load the initial coordinate inputs
    let pairs: Vec<(usize, usize)> = lines
        .iter()
        .map_while(|line| {
            // Split each line on ',' character, passing the two pieces into the match
            match line.as_ref().unwrap().split(',').collect::<Vec<&str>>()[..] {
                [a, b] => Some((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())),
                _ => None,
            }
        })
        .collect();

    // Build the grid
    let (max_cols, max_rows) = pairs
        .iter()
        .fold((0, 0), |(x, y), (a, b)| (cmp::max(x, *a), cmp::max(y, *b)));
    println!("pairs {:?}", pairs);
    println!("Max Columns {} Max Rows {}", max_cols, max_rows);
    let mut grid = Grid::new(max_rows + 1, max_cols + 1);
    for pair in pairs {
        grid.set(pair.1, pair.0, true).unwrap();
    }
    println!("grid {}", grid);

    // Parse the folds
    let folds: Vec<Fold> = lines
        .iter()
        .filter_map(|x| x.as_ref().unwrap().parse().ok())
        .collect();
    for fold in folds {
        println!("fold {}", fold);
        grid = grid.fold(fold);
        println!("grid {}", grid);
    }
}
