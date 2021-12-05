use array2d::Array2D;
use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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
    let marks: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_terminator(',')
        .filter_map(|v| v.parse::<usize>().ok())
        .collect();
    println!("{:?}", marks);

    let mut cards: Vec<Array2D<Option<usize>>> = Vec::new();
    for chunk in lines.collect::<Vec<_>>().chunks(6) {
        let card = Array2D::from_rows(
            &(chunk[..])
                .iter()
                .skip(1)
                .map(|row| {
                    row.as_ref()
                        .unwrap()
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        );
        cards.push(card);
    }
    println!("{:?}", cards);

    let n = 29;
    for mut card in cards {
        for (i, row) in card.clone().rows_iter().enumerate() {
            for (j, val) in row.enumerate() {
                if val.is_some() && n == val.unwrap() {
                    card.set(i, j, None).expect("Failed to mark number");
                    println!("bingo! {} {} {}", n, i, j);
                }
            }
        }
    }
    Ok(())
}
