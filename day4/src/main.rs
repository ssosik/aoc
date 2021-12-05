use array2d::Array2D;
use std::collections::BTreeSet;
use std::error;
use std::io::{BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug, Clone)]
struct Card(usize, Array2D<Option<usize>>);

impl Card {
    fn bingo(&self) -> bool {
        for row in self.1.as_rows() {
            if row.iter().all(|x| x.is_none()) {
                return true;
            }
        }
        for row in self.1.as_columns() {
            if row.iter().all(|x| x.is_none()) {
                return true;
            }
        }
        false
    }

    fn mark(&mut self, n: usize) -> Result<()> {
        for (i, row) in self.1.clone().rows_iter().enumerate() {
            for (j, val) in row.enumerate() {
                if val.is_some() && n == val.unwrap() {
                    self.1.set(i, j, None).expect("Failed to mark number");
                }
            }
        }
        Ok(())
    }

    fn sum(&self) -> Result<usize> {
        Ok(self.1.elements_row_major_iter().fold(0, |acc, x| {
            if x.is_some() {
                acc + x.unwrap()
            } else {
                acc
            }
        }))
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

    let mut cards: Vec<Card> = Vec::new();
    for (i, chunk) in lines.collect::<Vec<_>>().chunks(6).enumerate() {
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
        cards.push(Card(i, card));
    }

    let mut winning_cards: BTreeSet<usize> = BTreeSet::new();
    for n in marks {
        for card in &mut cards {
            let _ = &card.mark(n);
            //println!("WinningCards: {:?}", winning_cards);
            if winning_cards.contains(&card.0) {
                continue;
            }
            if card.bingo() {
                winning_cards.insert(card.0);
                println!(
                    "bingo! Mark:{} Card:{} Sum:{} Score:{} {:?}",
                    n,
                    card.0,
                    card.sum().unwrap(),
                    card.sum().unwrap() * n,
                    card.1,
                );
                //return Ok(());
            }
        }
    }
    Ok(())
}
