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

#[derive(Debug,Clone)]
struct Card(Array2D<Option<usize>>);

impl Card {
    fn bingo(&self) -> bool {
        //self.0.rows_iter().all(|x| x.is_none()) || self.0.columns_iter().all(|x| x.is_none())
        //self.0.rows_iter().inspect(|x| println!("X{:?}", x)).collect();
        //self.0.as_rows().iter().all(|x| x.is_none());
        for row in self.0.as_rows() {
            if row.iter().all(|x| x.is_none()) {
                return true;
            }
        }
        for row in self.0.as_columns() {
            if row.iter().all(|x| x.is_none()) {
                return true;
            }
        }
        false
    }
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

    let mut cards: Vec<Card> = Vec::new();
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
        cards.push(Card(card));
    }
    println!("{:?}", cards);

    let n = 29;
    for mut card in cards {
        for (i, row) in card.clone().0.rows_iter().enumerate() {
            for (j, val) in row.enumerate() {
                if val.is_some() && n == val.unwrap() {
                    card.0.set(i, j, None).expect("Failed to mark number");
                    if card.bingo() {
                        println!("bingo! {} {} {}", n, i, j);
                    }
                }
            }
        }
    }
    Ok(())
}
