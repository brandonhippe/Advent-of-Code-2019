use num::Integer;
use regex::Regex;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String, steps: i64) -> i64 {
    let int_regex = Regex::new(r"-?\d+").unwrap();
    let mut moons: Vec<Moon> = Vec::from(
        contents
            .lines()
            .map(|line| {
                let mut iter = int_regex.find_iter(line);
                Moon::new(
                    iter.next().unwrap().as_str().parse().unwrap(),
                    iter.next().unwrap().as_str().parse().unwrap(),
                    iter.next().unwrap().as_str().parse().unwrap(),
                )
            })
            .collect::<Vec<Moon>>(),
    );

    for _ in 0..steps {
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                if i != j {
                    let mut moon = moons[i].clone();
                    moon.gravity(&moons[j]);
                    moons[i] = moon;
                }
            }
        }

        for moon in moons.iter_mut() {

            moon.velocity();
        }
    }

    return moons.iter().map(|moon| moon.energy()).sum();
}

fn part2(contents: String) -> i64 {
    let int_regex = Regex::new(r"-?\d+").unwrap();

    let mut repeat_after: i64 = 1;

    for axis in 0..3 {
        let mut systems: Vec<System> = Vec::from(
            contents
                .lines()
                .map(|line| {
                    let mut matches = int_regex.find_iter(line);
                    let pos = matches.nth(axis).unwrap().as_str().parse::<i64>().unwrap();
                    System::new(pos, 0)
                })
                .collect::<Vec<System>>(),
        );

        let initial_state = systems.clone();
        let mut steps: i64 = 0;
        while steps == 0 || initial_state != systems {
            for i in 0..systems.len() {
                for j in 0..systems.len() {
                    if i != j {
                        let mut system = systems[i].clone();
                        system.gravity(&systems[j]);
                        systems[i] = system;
                    }
                }
            }

            for system in systems.iter_mut() {
                system.velocity();
            }

            steps += 1;
        }

        repeat_after = repeat_after.lcm(&steps);
    }

    return repeat_after;
}

#[derive(Debug, Clone)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn gravity(&mut self, other: &Moon) {
        if self.x < other.x {
            self.vx += 1;
        } else if self.x > other.x {
            self.vx -= 1;
        }

        if self.y < other.y {
            self.vy += 1;
        } else if self.y > other.y {
            self.vy -= 1;
        }

        if self.z < other.z {
            self.vz += 1;
        } else if self.z > other.z {
            self.vz -= 1;
        }
    }

    fn velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn energy(&self) -> i64 {
        (self.x.abs() + self.y.abs() + self.z.abs())
            * (self.vx.abs() + self.vy.abs() + self.vz.abs())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct System {
    pos: i64,
    vel: i64,
}

impl System {
    fn new(pos: i64, vel: i64) -> System {
        System { pos, vel }
    }

    fn gravity(&mut self, other: &System) {
        if self.pos < other.pos {
            self.vel += 1;
        } else if self.pos > other.pos {
            self.vel -= 1;
        }
    }

    fn velocity(&mut self) {
        self.pos += self.vel;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let mut contents =
            fs::read_to_string("example_1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 10), 179);

        contents =
            fs::read_to_string("example_2.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 100), 1940);
    }

    #[test]
    fn p2_test() {
        let mut contents =
            fs::read_to_string("example_1.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2772);

        contents =
            fs::read_to_string("example_2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 4686774924);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "12".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };


    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nTotal energy after 1000 steps: {}\nRan in {:.5?}",
        part1(contents.clone(), 1000),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRepeated position occurs at step: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}