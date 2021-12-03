use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::error;

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    const RADIX: u32 = 10;
    
    let input = BufReader::new(std::io::stdin());
    let lines = input.lines();

    let mut bit_counts: BTreeMap<usize, BTreeMap<u8, usize>> = BTreeMap::new();
    for line in lines {
        for (i, b) in line.unwrap().chars().enumerate() {
            let b = b.to_digit(RADIX).expect("to digit failed") as u8;
            bit_counts.entry(i)
                .and_modify(|m| {
                    m.entry(b)
                    .and_modify(|x| *x += 1)
                    //.or_insert(BTreeMap::from([(b, 1)]));
                    .or_insert(1);
                })
                .or_insert(BTreeMap::from([(b, 1)]));
            print!("{}:{} ", i, b);
        }
        println!("");
    };

    println!("{:?}", bit_counts);
    Ok(())
}
