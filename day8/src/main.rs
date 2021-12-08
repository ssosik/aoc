use std::io::{BufRead, BufReader};

fn main() {
    let mut cnt1478 = 0;
    let input = BufReader::new(std::io::stdin());
    for line in input.lines() {
        let (all_digits, display) = match line.unwrap().split(" | ").collect::<Vec<&str>>()[..] {
            [all, out] => {
                let (all, out) = (
                    all.to_string()
                        .split_whitespace()
                        .map(|n| n.parse::<String>().ok())
                        .collect::<Vec<_>>(),
                    out.to_string()
                        .split_whitespace()
                        .map(|n| n.parse::<String>().ok())
                        .collect::<Vec<_>>(),
                );
                cnt1478 += out
                    .iter()
                    .map(|e| match e.as_ref().unwrap().len() {
                        2 | 3 | 4 | 7 => 1,
                        _ => 0,
                    })
                    .sum::<usize>();
                (all, out)
            }
            _ => unreachable!(),
        };
        println!("All {:?}; Out {:?}", all_digits, display);
    }
    println!("Cnt: {}", cnt1478);
}
