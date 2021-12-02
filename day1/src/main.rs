use std::io::{BufRead, BufReader};

fn main() {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();

    let mut last: Option<usize> = None;
    let mut cnt = 0;
    for line in lines {
        // Get the next value
        let a: Vec<usize> = line
            .unwrap()
            .split_whitespace()
            .filter_map(|v| v.parse::<usize>().ok())
            .collect();
        let a = a[0];

        if last.is_some() && a > last.unwrap() {
            cnt += 1;
        }

        last = Some(a);
    }
    println!("{}", cnt);
}
