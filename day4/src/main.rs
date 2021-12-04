use std::io::{BufRead, BufReader};
use std::error;
use array2d::Array2D;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
#[derive(Debug)]
struct Card {
    rows: [[i32; 5]; 5],
}

//impl Card {
//    fn new() -> Submarine {
//        Submarine{position: 0, depth: 0, aim: 0}
//    }
//}

fn main() -> Result<()> {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();
    let marks: Vec<usize> = lines.next().unwrap().unwrap()
            .split_terminator(",")
            .filter_map(|v| v.parse::<usize>().ok())
            .collect();
    println!("{:?}", marks);

    //let card: Array2D<usize>;
    //lines.next();
    //let rows: Vec<Vec<&str>> = lines.take(5).collect();
    //println!("5 rows{:?}", rows);

    //lines.skip(1).take(5).inspect(|x| println!("{:?}", x)).collect::<Vec<_>>();
    loop {
    lines.by_ref()
        .skip(1)
        .take(5)
        .inspect(|x| println!("x{:?}", x))
        .collect::<Vec<_>>();
        //.map(|x| x.unwrap().split_whitespace()
        //    .filter_map(|v| v.parse::<usize>().ok()));
        }

    //for line in lines {
    //    let line: Vec<usize> = line
    //        .unwrap()
    //        .split_whitespace()
    //        .filter_map(|v| v.parse::<usize>().ok())
    //        .collect();
    //    if line.len() == 0 {
    //        
    //    }
    //    println!("{:?}", line);
    //}

    Ok(())
}
