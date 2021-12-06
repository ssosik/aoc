use std::io::{BufRead, BufReader};
use futures::future::try_join_all;
use futures::Future;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[tokio::main]
async fn main() {
    let n = 80;
    let input = BufReader::new(std::io::stdin())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse::<u8>().ok())
        .collect::<Vec<u8>>();
    let jobs = input.iter().map(|x| breed(*x, n));
    let nums: Vec<usize> = try_join_all(jobs).await.unwrap();
    //let jobs: Vec<dyn Future<Output = usize>> = Vec::with_capacity(100);
    println!("input {:?} {}", nums, nums.iter().fold(0, |acc, x| x + acc));
}

async fn breed(s: u8, n: u16) -> Result<usize> {
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
    Ok(input.len())
}
