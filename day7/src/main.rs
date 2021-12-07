use std::cmp;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};

fn main() {
    let positions: Vec<isize> = BufReader::new(std::io::stdin())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .filter_map(|v| v.parse::<isize>().ok())
        .collect();
    //println!("positions {:?}", positions);

    let max = positions.iter().max().unwrap();
    let mut costs: BTreeMap<isize, isize> = (0..=*max).fold(BTreeMap::new(), |mut acc, x| {
        acc.insert(x, 0);
        acc
    });
    //println!("costs {:?}", costs);

    for pos in positions {
        let mut tmp: BTreeMap<isize, isize> = BTreeMap::new();
        for (cost, cnt) in &costs {
            let sum = (1..=(pos - cost).abs())
                .enumerate()
                //.inspect(|x| println!("X:{:?}", x))
                .fold(0, |acc, (_i, p)| acc + p as isize);
            //println!("cost {} cnt {} positon {} sum {}", cost, cnt, pos, sum);
            tmp.insert(*cost, cnt + sum);
        }
        costs.append(&mut tmp);
    }

    println!("costs {:?}", costs);
    let min = costs
        .iter()
        .fold(isize::MAX, |acc, (_, v)| cmp::min(acc, *v));
    println!("Min {}", min);
}
