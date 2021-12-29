use derive_more::Display;
use std::cmp;
use std::ops::Range;

#[derive(Debug, Display)]
#[display(fmt = "({},{}) {}", "x", "y", "status")]
struct Projectile {
    x: isize,
    y: isize,
    max_y: isize,
    velocity_x: isize,
    velocity_y: isize,
    target_x: Range<isize>,
    target_y: Range<isize>,
    status: FlightStatus,
}

#[derive(Debug, Display, PartialEq)]
enum FlightStatus {
    Unreached,
    Hit,
    Past,
}

impl Projectile {
    fn step(&mut self) -> bool {
        println!("Step {}", self);
        self.x += self.velocity_x;
        let new_y = self.y + self.velocity_y;
        self.velocity_x += match self.velocity_x {
            x if x > 0 => -1,
            x if x < 0 => 1,
            _ => 0,
        };
        self.velocity_y -= 1;
        self.max_y = cmp::max(self.y, new_y);
        self.y = new_y;

        if self.target_x.contains(&self.x) && self.target_y.contains(&self.y) {
            self.status = FlightStatus::Hit;
        } else if self.target_x.end < self.x || self.target_y.end > self.y {
            self.status = FlightStatus::Past;
        }

        true
    }
}

fn main() {
    let mut p = Projectile {
        x: 0,
        y: 0,
        max_y: 0,
        velocity_x: 5,
        velocity_y: 5,
        target_x: 20..30,
        target_y: -10..-5,
        status: FlightStatus::Unreached,
    };

    while p.status == FlightStatus::Unreached {
        p.step();
    }
    println!("Projectile {:?}", p);
}
