use derive_more::Display;
use std::cmp;
use std::ops::Range;

#[derive(Debug, Display)]
#[display(fmt = "Loc:({},{}) {} MaxY {}", "x", "y", "status", "max_y")]
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
        //println!("Step {}", self);
        self.x += self.velocity_x;
        self.y += self.velocity_y;
        self.velocity_x += match self.velocity_x {
            x if x > 0 => -1,
            x if x < 0 => 1,
            _ => 0,
        };
        self.velocity_y -= 1;
        self.max_y = cmp::max(self.y, self.max_y);

        if self.target_x.contains(&self.x) && self.target_y.contains(&self.y) {
            self.status = FlightStatus::Hit;
        } else if self.target_x.end < self.x || self.target_y.end > self.y {
            self.status = FlightStatus::Past;
        }

        true
    }
}

fn main() {
    let mut max = 0;
    for vel_y in 0..200 {
        for vel_x in 0..200 {
            let mut p = Projectile {
                x: 0,
                y: 0,
                max_y: max,
                velocity_x: vel_x,
                velocity_y: vel_y,
                target_x: 20..30,
                //target_x: 265..287,
                target_y: -10..-5,
                //target_y: -103..-58,
                status: FlightStatus::Unreached,
            };

            while p.status == FlightStatus::Unreached {
                p.step();
            }
            if p.status == FlightStatus::Hit {
                println!("Projectile {}", p);
                max = cmp::max(max, p.max_y);
            }
        }
    }
}
