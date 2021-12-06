use array2d::Array2D;

use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    //let mut grid = Array2D::filled_with(0, 1000, 1000);
    let mut grid = Array2D::filled_with(0, 10, 10);

    let input = BufReader::new(std::io::stdin());
    for line in input.lines() {
        let (x1, y1, x2, y2) = match line.unwrap().split(" -> ").collect::<Vec<&str>>()[..] {
            [start, end] => {
                let (x1, y1) = match start
                    .split(',')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<usize>>()[..]
                {
                    [x1, y1] => (x1, y1),
                    _ => unreachable!(),
                };
                let (x2, y2) = match end
                    .split(',')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<usize>>()[..]
                {
                    [x2, y2] => (x2, y2),
                    _ => unreachable!(),
                };
                (x1, y1, x2, y2)
            }
            _ => unreachable!(),
        };
        println!("x1:{} y1:{} x2:{} y2:{}", x1, y1, x2, y2);
        if x1 == x2 {
            if y1 < y2 {
                for y in y1..=y2 {
                    grid.get_mut(y, x1).map(|n| *n += 1);
                }
            } else {
                for y in y2..=y1 {
                    grid.get_mut(y, x1).map(|n| *n += 1);
                }
            }
        //} else {
        } else if y1 == y2 {
            if x1 < 2 {
                for x in x1..=x2 {
                    grid.get_mut(y1, x).map(|n| *n += 1);
                }
            } else {
                for x in x2..=x1 {
                    grid.get_mut(y1, x).map(|n| *n += 1);
                }
            }
        } else {
            println!("Diagonal {} {} {} {}", x1, y1, x2, y2);
            //    unreachable!()
        };
    }
    let mut cnt = 0;
    //println!("Grid: {:?}", grid);
    for row in grid.rows_iter() {
        //println!("{:?}", row.into_iter().collect::<Vec<usize>>());
        for item in row {
            if item == &0 {
                print!(".");
            } else {
                print!("{}", item);
            }
            if item > &1 {
                cnt += 1;
            }
        }
        println!("");
    }
    let overlap_cnt: usize = grid
        .as_row_major()
        .into_iter()
        .fold(0, |acc, x| acc + if x > 1 { 1 } else { 0 });
    println!("Overlap Count: {} {}", overlap_cnt, cnt);

    Ok(())
}
