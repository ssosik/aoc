use std::io::{BufRead, BufReader};

fn main() {
    let entries: Vec<String> = BufReader::new(std::io::stdin())
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split('|')
        //.filter_map(|v| v.parse::<isize>().ok())
        .filter_map(|v| Some(v.to_string()))
        .collect();
    println!("entries {:?}", entries);

    //for line in input.lines() {
    //    let (x1, y1, x2, y2) = match line.unwrap().split(" -> ").collect::<Vec<&str>>()[..] {
    //        [start, end] => {
    //            let (x1, y1) = match start
    //                .split(',')
    //                .filter_map(|v| v.parse::<usize>().ok())
    //                .collect::<Vec<usize>>()[..]
    //            {
    //                [x1, y1] => (x1, y1),
    //                _ => unreachable!(),
    //            };
    //            let (x2, y2) = match end
    //                .split(',')
    //                .filter_map(|v| v.parse::<usize>().ok())
    //                .collect::<Vec<usize>>()[..]
    //            {
    //                [x2, y2] => (x2, y2),
    //                _ => unreachable!(),
    //            };
    //            (x1, y1, x2, y2)
    //        }
    //        _ => unreachable!(),
    //    };
    //};
}
