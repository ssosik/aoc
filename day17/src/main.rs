use derive_more::Display;
use std::cmp;
use std::ops::RangeInclusive;

#[derive(Debug, Display)]
#[display(fmt = "{},{}", "initial_velocity_x", "initial_velocity_y")]
struct Projectile {
    x: isize,
    y: isize,
    max_y: isize,
    initial_velocity_x: isize,
    initial_velocity_y: isize,
    velocity_x: isize,
    velocity_y: isize,
    target_x: RangeInclusive<isize>,
    target_y: RangeInclusive<isize>,
    status: FlightStatus,
}

#[derive(Debug, Display, PartialEq)]
enum FlightStatus {
    Unreached,
    Hit,
    Past,
}

impl Projectile {
    fn new(
        vel_x: isize,
        vel_y: isize,
        targ_x: RangeInclusive<isize>,
        targ_y: RangeInclusive<isize>,
    ) -> Projectile {
        Projectile {
            x: 0,
            y: 0,
            max_y: 0,
            initial_velocity_x: vel_x,
            initial_velocity_y: vel_y,
            velocity_x: vel_x,
            velocity_y: vel_y,
            target_x: targ_x,
            target_y: targ_y,
            status: FlightStatus::Unreached,
        }
    }
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

        //println!("Current Location {} {}", self.x, self.y);
        if self.target_x.contains(&self.x) && self.target_y.contains(&self.y) {
            //println!("Hit at Location {} {}", self.x, self.y);
            self.status = FlightStatus::Hit;
        } else if self.target_x.end() < &self.x || self.target_y.start() > &self.y {
            //println!("PAST at Location {} {}", self.x, self.y);
            self.status = FlightStatus::Past;
        }

        true
    }
}

fn main() {
    let mut max = 0;
    let mut hit_cnt = 0;
    for vel_y in -1000..1000 {
        for vel_x in 0..1000 {
            //let mut p = Projectile::new(vel_x, vel_y, 20..=30, -10..=-5);
            let mut p = Projectile::new(vel_x, vel_y, 265..=287, -103..=-58);

            //println!("Testing projectile {}", p);
            while p.status == FlightStatus::Unreached {
                p.step();
            }
            if p.status == FlightStatus::Hit {
                println!("{}", p);
                max = cmp::max(max, p.max_y);
                hit_cnt += 1;
            }
        }
    }
    println!("HitCnt {}", hit_cnt);
}
