use std::io::{BufRead, BufReader};

fn main() {
    let n = 18;
    let sum: usize = BufReader::new(std::io::stdin())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|v| breed(v.parse::<u8>().unwrap(), n))
        .sum();
    println!("{:>5}: input {:?}", 0, sum);
    //for i in 1..=n {
    //    let mut tmp: Vec<usize> = Vec::new();
    //    let mut babies: Vec<usize> = Vec::new();
    //    for t in input.clone() {
    //        tmp.push(match t {
    //            0 => {
    //                babies.push(8);
    //                6
    //            }
    //            _ => t - 1,
    //        });
    //    }
    //    input = tmp;
    //    input.extend(babies);
    //    //println!("{:>5}: input {:?}", i, input);
    //    println!("{}", i);
    //}
    //println!("Len: {}", input.len());
}

fn breed(s: u8, n: u16) -> usize {
    let mut input = vec![s];

    for i in 1..=n {
        let mut tmp: Vec<u8> = Vec::new();
        let mut babies: Vec<u8> = Vec::new();
        for t in input.clone() {
            tmp.push(match t {
                0 => {
                    babies.push(8);
                    6
                }
                _ => t - 1,
            });
        }
        input = tmp;
        input.extend(babies);
        //println!("{:>5}: input {:?}", i, input);
        println!("{}", i);
    }
    //println!("Len: {}", input.len());
    input.len()
}
