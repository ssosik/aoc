#![feature(iter_map_while)]

use array2d::Array2D;
use itertools::{EitherOrBoth::*, Itertools};
use std::cmp;
use std::fmt;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Grid(Array2D<bool>);

impl Grid {
    fn new(rows: usize, cols: usize) -> Grid {
        Grid(Array2D::filled_with(false, rows, cols))
    }
    fn fold(&self, fold: Fold) -> Grid {
        let (get_slices, n) = match fold {
            Fold::Horizontal(n) => (self.0.as_rows(), n),
            Fold::Vertical(n) => (self.0.as_columns(), n),
        };
        // Zip the REVERSED first half, up to Row N....
        let slices = &get_slices[..n]
            .iter()
            .rev() // Reverse this list so that the fold lines up exactly with the second half
            // ... with the second half, from Row N+1 to the end
            .zip_longest(get_slices[n + 1..].iter())
            // Handle different lengths of the halves
            .map(|slices| match slices {
                // Both iterators have a slice, zip the slice items together
                Both(l, r) => l
                    .iter()
                    .zip(r)
                    // Merge the two items together, boolean OR.
                    .map(|(a, b)| *a || *b)
                    .collect::<Vec<bool>>(),
                // The second half iterator has no more slices, take
                // the remaining slices from the first half
                Left(l) => l.to_vec(),
                // Don't need to handle when the second half folds
                // beyond the first half
                Right(_) => unreachable!(),
            })
            // Re-Reverse the zipped grid to put the slices back into original order
            .rev()
            .collect::<Vec<Vec<bool>>>();
        match fold {
            Fold::Horizontal(_) => Grid(Array2D::from_rows(slices)),
            Fold::Vertical(_) => Grid(Array2D::from_columns(slices)),
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

    // Find the maximum row and columns
    let (max_cols, max_rows) = pairs
        .iter()
        .fold((0, 0), |(x, y), (a, b)| (cmp::max(x, *a), cmp::max(y, *b)));
    let (max_cols, max_rows) = (max_cols + 1, max_rows + 1);
    // Build the grid
    let mut grid = Grid::new(max_rows, max_cols);
    for pair in pairs {
        grid.set(pair.1, pair.0, true).unwrap();
    }
    //println!("grid {}", grid);

    // Parse and process the folds
    for fold in lines
        .iter()
        .filter_map(|x| x.as_ref().unwrap().parse().ok())
        .collect::<Vec<Fold>>()
    {
        println!("fold {}", fold);
        grid = grid.fold(fold);
        println!("grid {}", grid);
        //let num = grid.0.elements_row_major_iter().filter(|&x| *x).count();
        //println!("num {}", num);
    }
}
