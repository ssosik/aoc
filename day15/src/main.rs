use array2d::Array2D;
use pathfinding::prelude::{absdiff, astar};
use std::cmp;
use std::fmt;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize, u32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (absdiff(self.0, other.0) + absdiff(self.1, other.1)) as u32
    }
}

#[derive(Debug, Clone)]
struct Grid(Array2D<u32>);

impl Grid {
    fn new(lines: &Vec<Vec<u32>>) -> Grid {
        Grid(Array2D::from_rows(lines))
    }
    fn neighbors(&self, p: &Pos) -> Vec<(Pos, u32)> {
            let &Pos(x, y, _) = p;
            println!("HERE {} {}", x, y);
            //vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
            //     Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
            //     .into_iter().map(|p| (p, 1)).collect()
            match (x, y) {
                (0, 0) => println!("upper left"),
                (0, 1) => println!("upper right"),
                (1, 0) => println!("lower left"),
                (1, 1) => println!("lower right"),
                (a, b) => println!("mid {} {}", a, b),
            };
            vec![(Pos(1, 2, 3), 4)]
    }
}

fn main() {
    let lines: Vec<Vec<u32>> = BufReader::new(std::io::stdin())
        .lines()
        //.map(|l| l.unwrap().chars().map(|c| c.to_digit(10_u32).unwrap()))
        .map(|l| l.unwrap().chars().collect::<_>())
        .map(|l: Vec<char>| {
            l.iter()
                .map(|c| c.to_digit(10_u32).unwrap())
                .collect::<Vec<u32>>()
        })
        .inspect(|x| println!("X {:?}", x))
        .collect();
    let grid = Grid::new(&lines);
    let rows = grid.0.num_rows();
    let columns = grid.0.num_columns();

    let start = Pos(0, 0, *grid.0.get(0, 0).unwrap());
    let goal: Pos = Pos(
        rows - 1,
        columns - 1,
        *grid.0.get(rows - 1, columns - 1).unwrap(),
    );

    println!("lines: {:?}", lines);
    println!("rows: {} columns {}", rows, columns);
    println!("goal {}", grid.0.get(rows - 1, columns - 1).unwrap());

    let result = astar(&start,
        |p| grid.neighbors(p),
        |p| p.distance(&goal) / 3,
        |p| *p == goal
        );
    println!("result: {:?}", result);
    
}
