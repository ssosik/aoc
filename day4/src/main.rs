use array2d::Array2D;
use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug, Clone)]
struct Card(Array2D<Option<usize>>);

impl Card {
    fn bingo(&self) -> bool {
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

    fn mark(&mut self, n: usize) -> Result<()> {
        for (i, row) in self.0.clone().rows_iter().enumerate() {
            for (j, val) in row.enumerate() {
                if val.is_some() && n == val.unwrap() {
                    self.0.set(i, j, None).expect("Failed to mark number");
                }
            }
        }
        Ok(())
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
    //println!("{:?}", cards);

    for n in marks {
        //println!("N:{}", n);
        for card in &mut cards {
            let _ = &card.mark(n);
            if card.bingo() {
                println!("bingo! {} {:?}", n, card);
                return Ok(());
            }
        }
    }
    Ok(())
}
