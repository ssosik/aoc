use std::collections::{BTreeMap, BTreeSet};
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
    let mut numbers = BufReader::new(std::io::stdin())
        .lines()
        .map(|x| usize::from_str_radix(x.unwrap().as_str(), 2).unwrap())
        .collect::<Vec<usize>>();

    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");
    let mut bit_counts: BTreeMap<usize, BTreeMap<u8, usize>> = BTreeMap::new();

    let mut bit_idx = 0;
    let mut msb1_mask = 31;
    while !&numbers.is_empty() {
        // Find the larger bit count for the current bit_idx
        let mut cnt0: usize = 0;
        let mut cnt1: usize = 0;
        for n in &numbers {
            let bin_str = format!("{:05b}", n);
            print!("{} ", bin_str);
            match bin_str.chars().collect::<Vec<char>>()[bit_idx].to_digit(2).unwrap() {
                0 => cnt0 += 1,
                1 => cnt1 += 1,
                _ => unreachable!(),
            };
        }
        println!("");
        msb1_mask = if cnt0 > cnt1 {
            msb1_mask | (0 << (4 - bit_idx))
        } else {
            msb1_mask | (1 << (4 - bit_idx))
        };

        println!("Mask: {:05b} ({})", msb1_mask, msb1_mask);

        let mut to_filter: BTreeSet<usize> = BTreeSet::new();
        for n in &numbers {
            to_filter.insert(*n);
        }

        for n in &numbers {

            let masked1 = n & msb1_mask;
            if masked1 == 0 {
                to_filter.remove(n);
            }
            println!("CNT1 Mask {} {} {:05b}", n, masked1, msb1_mask);
        }

        println!("filtered {:?}", to_filter);
        numbers = to_filter.iter().map(|x| *x).collect();


        //match cnt0 > cnt1 {
        //    true => {
        //       println!("cnt0: {} {}", cnt0, 1 << (4 - bit_idx));
        //    },
        //    false => {
        //       println!("cnt1: {} {}", cnt1, 1 << (4 - bit_idx));
        //    },
        //};
        //println!("cnt0: {}, 1:{}, 2:{}, 3:{}, 4:{}, 5:{}",
        //    cnt0, cnt0 << 1, cnt0 << 2, cnt0 << 3, cnt0 << 4, cnt0 << 5);
        //println!("cnt1: {}, 1:{}, 2:{}, 3:{}, 4:{}, 5:{}",
        //    cnt1, cnt1 << 1, cnt1 << 2, cnt1 << 3, cnt1 << 4, cnt1 << 5);
        //numbers = vec![];
        bit_idx += 1;
    }

    //for line in numbers.clone() {
    //    //let bin_str = format!("{:012b}", line);
    //    let bin_str = format!("{:05b}", line);
    //    println!("{} ", bin_str);

    //    for (idx, bit) in bin_str.chars().enumerate() {
    //        let bit = bit.to_digit(RADIX).expect("to digit failed") as u8;
    //        bit_counts
    //            .entry(idx)
    //            .and_modify(|m| {
    //                m.entry(bit).and_modify(|cnt| *cnt += 1).or_insert(1);
    //            })
    //            .or_insert(BTreeMap::from([(bit, 1)]));
    //    }
    //}

    //println!("{:?}", bit_counts);

    //for i in bit_counts.keys() {
    //    match bit_counts[i].get(&0).or(Some(&0)) > bit_counts[i].get(&1).or(Some(&0)) {
    //        true => {
    //            gamma_rate.push_str("0");
    //            epsilon_rate.push_str("1");
    //        }
    //        false => {
    //            gamma_rate.push_str("1");
    //            epsilon_rate.push_str("0");
    //        }
    //    };
    //}

    //println!("Gamma:{} Epsilon:{}", gamma_rate, epsilon_rate);
    //let gamma_rate = isize::from_str_radix(gamma_rate.as_str(), 2).unwrap();
    //let epsilon_rate = isize::from_str_radix(epsilon_rate.as_str(), 2).unwrap();
    //println!("Gamma:{} Epsilon:{}", gamma_rate, epsilon_rate);
    //println!("Ans:{}", gamma_rate * epsilon_rate);

    //for line in numbers {
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
