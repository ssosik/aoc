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

fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() -> Result<()> {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();
    let marks: Vec<usize> = lines.next().unwrap().unwrap()
            .split_terminator(",")
            .filter_map(|v| v.parse::<usize>().ok())
            .collect();
    println!("{:?}", marks);

    for chunk in lines.collect::<Vec<_>>().chunks(6) {
        let card = Array2D::from_rows(&(chunk[..])
            .iter()
            .skip(1)
            .map(|row| {
                row.as_ref()
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>());
        println!("{:?}", card);
    }

    Ok(())
}
