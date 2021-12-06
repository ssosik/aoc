use std::io::{BufRead, BufReader};
use futures::future::{join_all, try_join_all};
use futures::Future;
use std::thread;
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
    let mut jobs = Vec::new();
    for x in input {
     let thread = thread::spawn(move || breed(x, n.clone()));
     jobs.push(thread);
    }
    let mut sum = 0;
    for job in jobs {
        sum += job.join().unwrap();
    }
    //let sum = jobs.iter().fold(0, |acc, x| &x.clone().join().unwrap() + acc);
    //let jobs: Vec<dyn Future<Output = usize>> = Vec::with_capacity(100);
    println!("sum {}", sum);
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
    println!("Len: {}", input.len());
    input.len()
}
