use std::io::{BufRead, BufReader};
use std::error;
use array2d::Array2D;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
#[derive(Debug)]
struct Card {
    rows: [[i32; 5]; 5],
}

//impl Card {
//    fn new() -> Submarine {
//        Submarine{position: 0, depth: 0, aim: 0}
//    }
//}

fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() -> Result<()> {
    let input = BufReader::new(std::io::stdin());
    let mut lines = input.lines();
    let marks: Vec<usize> = lines.next().unwrap().unwrap()
            .split_terminator(",")
            .filter_map(|v| v.parse::<usize>().ok())
            .collect();
    println!("{:?}", marks);

    for chunk in lines.collect::<Vec<_>>().chunks(6) {
        //println!("{:?} {:?}", chunk, print_type_of(chunk));
        //println!("{:?}", &chunk[1..]);
        let it = &(chunk[1..])
            .iter()
            .map(|x| {
                x.as_ref().unwrap()
            })
            .collect::<Vec<_>>();
        println!("{:?}", it);
        //let it = chunk.skip(1).map(|row| row.unwrap()).collect();
        //println!("{:?}", chunk[1..]);
    }

    //let card: Array2D<usize>;
    //lines.next();
    //let rows: Vec<Vec<&str>> = lines.take(5).collect();
    //println!("5 rows{:?}", rows);

    //lines.skip(1).take(5).inspect(|x| println!("{:?}", x)).collect::<Vec<_>>();
    //loop {
    //    lines.by_ref()
    //        .skip(1)
    //        .take(5)
    //        .inspect(|x| println!("x{:?}", x))
    //        .collect::<Vec<_>>();
    //        //.map(|x| x.unwrap().split_whitespace()
    //        //    .filter_map(|v| v.parse::<usize>().ok()));
    //    if lines.by_ref().is_empty() {
    //        break;
    //        }
    //    //if lines.by_ref().peekable().peek().is_none() {
    //    //    break;
    //    //}
    //}

    //for line in lines {
    //    let line: Vec<usize> = line
    //        .unwrap()
    //        .split_whitespace()
    //        .filter_map(|v| v.parse::<usize>().ok())
    //        .collect();
    //    if line.len() == 0 {
    //        
    //    }
    //    println!("{:?}", line);
    //}

    Ok(())
}
