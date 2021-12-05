use array2d::Array2D;

use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let _grid = Array2D::filled_with(0, 1000, 1000);

    let input = BufReader::new(std::io::stdin());
    let lines = input.lines();
    for line in lines {
        //println!("line {:?}", line);
        //let (start, end) = line.unwrap().split(" -> ").collect::<Vec<String>>()[..];
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
    }

    Ok(())
}
