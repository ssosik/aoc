use std::cmp;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    let positions: Vec<isize> = BufReader::new(std::io::stdin())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse::<isize>().ok())
        .collect();
    println!("positions {:?}", positions);

    let mut costs: BTreeMap<isize, isize> = positions.iter().fold(BTreeMap::new(), |mut acc, x| {
        acc.insert(*x, 0);
        acc
    });
    println!("positions {:?}", costs);

    for pos in positions {
        let mut tmp: BTreeMap<isize, isize> = BTreeMap::new();
        for (cost, cnt) in &costs {
            //println!("cost {} cnt {} positon {}", cost, cnt, pos);
            tmp.insert(*cost, cnt + (pos - cost).abs());
        }
        costs.append(&mut tmp);
    }

    println!("positions {:?}", costs);
    let min = costs
        .iter()
        .fold(isize::MAX, |acc, (_, v)| cmp::min(acc, *v));
    println!("Min {}", min);
}
