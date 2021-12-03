use std::collections::BTreeMap;
use std::error;
use std::io::{BufRead, BufReader};

// 00100
// 11110
// 10110
// 10111
// 10101
// 01111
// 00111
// 11100
// 10000
// 11001
// 00010
// 01010

// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
const RADIX: u32 = 10;

fn main() -> Result<()> {
    let lines = BufReader::new(std::io::stdin())
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| usize::from_str_radix(x.unwrap().as_str(), 2).unwrap())
        .collect::<Vec<usize>>();

    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");
    let mut bit_counts: BTreeMap<usize, BTreeMap<u8, usize>> = BTreeMap::new();

    for line in lines.clone() {
        //let bin_str = format!("{:012b}", line);
        let bin_str = format!("{:05b}", line);
        //println!("{} ", bin_str);

        for (idx, bit) in bin_str.chars().enumerate() {
            let bit = bit.to_digit(RADIX).expect("to digit failed") as u8;
            bit_counts
                .entry(idx)
                .and_modify(|m| {
                    m.entry(bit).and_modify(|cnt| *cnt += 1).or_insert(1);
                })
                .or_insert(BTreeMap::from([(bit, 1)]));
        }
    }

    println!("{:?}", bit_counts);

    for i in bit_counts.keys() {
        match bit_counts[i].get(&0).or(Some(&0)) > bit_counts[i].get(&1).or(Some(&0)) {
            true => {
                gamma_rate.push_str("0");
                epsilon_rate.push_str("1");
            }
            false => {
                gamma_rate.push_str("1");
                epsilon_rate.push_str("0");
            }
        };
    }

    println!("Gamma:{} Epsilon:{}", gamma_rate, epsilon_rate);
    let gamma_rate = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    let epsilon_rate = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();
    println!("Gamma:{} Epsilon:{}", gamma_rate, epsilon_rate);
    println!("Ans:{}", gamma_rate * epsilon_rate);

    //for line in lines {
    //    for (idx, bit) in line.chars().enumerate() {
    //        let bit = bit.to_digit(RADIX).expect("to digit failed") as u8;
    //        print!("{}:{} ", idx, bit);
    //    }
    //    println!("");
    //}

    let test = 5 | 2;

    println!("Test: {}", test);
    Ok(())
}
