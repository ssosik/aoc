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

fn main() -> Result<()> {
    let mut numbers = BufReader::new(std::io::stdin())
        .lines()
        .map(|x| usize::from_str_radix(x.unwrap().as_str(), 2).unwrap())
        .collect::<Vec<usize>>();
    let numbers_clone = numbers.clone();

    let word_len = 12;

    let mut mask: Vec<Option<usize>> = vec![None; word_len];
    for bit_idx in 0..word_len {
        // Find the larger bit count for the current bit_idx
        let mut cnt0: usize = 0;
        let mut cnt1: usize = 0;
        for n in &numbers {
            let bin_str = format!("{:012b}", n);
            //print!("{}:{} ", n, bin_str);
            match bin_str.chars().collect::<Vec<char>>()[bit_idx].to_digit(2).unwrap() {
                0 => cnt0 += 1,
                1 => cnt1 += 1,
                _ => unreachable!(),
            };
        }
        if cnt0 > cnt1 {
            mask[bit_idx] = Some(0);
        } else {
            mask[bit_idx] = Some(1);
        }
        //println!("Mask: {:?}", mask);

        let mut tmp: Vec<usize> = Vec::new();
        for n in &numbers {
            let nstr = format!("{:012b}", n);
            let mut discard = false;
            for (idx, bit) in nstr.chars().enumerate() {
                let bit = bit.to_digit(10_u32).expect("to digit failed") as usize;
                if mask[idx].is_some() && mask[idx].unwrap() != bit {
                    discard = true;
                    break;
                }
            }
            if !discard {
                tmp.push(*n);
            }

        }
        numbers = tmp;
        if numbers.len() < 2 {
            break;
        }
    }
    println!("Oxygen {:?}", numbers);

    numbers = numbers_clone;
    let mut mask: Vec<Option<usize>> = vec![None; word_len];
    for bit_idx in 0..word_len {
        // Find the larger bit count for the current bit_idx
        let mut cnt0: usize = 0;
        let mut cnt1: usize = 0;
        for n in &numbers {
            let bin_str = format!("{:012b}", n);
            //print!("{}:{} ", n, bin_str);
            match bin_str.chars().collect::<Vec<char>>()[bit_idx].to_digit(2).unwrap() {
                0 => cnt0 += 1,
                1 => cnt1 += 1,
                _ => unreachable!(),
            };
        }
        if cnt0 > cnt1 {
            mask[bit_idx] = Some(1);
        } else {
            mask[bit_idx] = Some(0);
        }
        //println!("Mask: {:?}", mask);

        let mut tmp: Vec<usize> = Vec::new();
        for n in &numbers {
            let nstr = format!("{:012b}", n);
            let mut discard = false;
            for (idx, bit) in nstr.chars().enumerate() {
                let bit = bit.to_digit(10_u32).expect("to digit failed") as usize;
                if mask[idx].is_some() && mask[idx].unwrap() != bit {
                    discard = true;
                    break;
                }
            }
            if !discard {
                tmp.push(*n);
            }

        }
        numbers = tmp;
        if numbers.len() < 2 {
            break;
        }
    }
    println!("CO2 {:?}", numbers);

    Ok(())
}
