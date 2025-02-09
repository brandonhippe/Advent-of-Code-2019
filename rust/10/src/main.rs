use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut sees: HashMap<(i64, i64), HashMap<(i64, i64), HashSet<(i64, i64)>>> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                sees.insert((x as i64, y as i64), HashMap::new());
            }
        }
    }

    for p1 in sees.clone().keys() {
        for p2 in sees.clone().keys() {
            if p1 == p2 {
                continue;
            }

            let (x1, y1) = p1;
            let (x2, y2) = p2;

            let mut dx = x2 - x1;
            let mut dy = y2 - y1;
            let gcd = if dx.abs() > dy.abs() {
                euclids_extended_algorithm(dx, dy).0.abs()
            } else {
                euclids_extended_algorithm(dy, dx).0.abs()
            };

            dx /= gcd;
            dy /= gcd;

            let entry = sees.get_mut(p1).unwrap();
            let set = entry.entry((dx, dy)).or_insert(HashSet::new());
            set.insert(*p2);
        }
    }

    return sees.values().map(|x| x.len() as i64).max().unwrap();
}

fn part2(contents: String) -> i64 {
    let mut sees: HashMap<(i64, i64), HashMap<(i64, i64), HashSet<(i64, i64)>>> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                sees.insert((x as i64, y as i64), HashMap::new());
            }
        }
    }

    for p1 in sees.clone().keys() {
        for p2 in sees.clone().keys() {
            if p1 == p2 {
                continue;
            }

            let (x1, y1) = p1;
            let (x2, y2) = p2;

            let mut dx = x2 - x1;
            let mut dy = y2 - y1;
            let gcd = if dx.abs() > dy.abs() {
                euclids_extended_algorithm(dx, dy).0.abs()
            } else {
                euclids_extended_algorithm(dy, dx).0.abs()
            };
            dx /= gcd;
            dy /= gcd;

            let entry = sees.get_mut(p1).unwrap();
            let set = entry.entry((dx, dy)).or_insert(HashSet::new());
            set.insert(*p2);
        }
    }

    let mut max_spot = (0, 0);
    let mut max_sees = 0;
    for (spot, map) in sees.iter() {
        if map.len() > max_sees {
            max_spot = *spot;
            max_sees = map.len();
        }
    }

    let mut dir_vec: Vec<(i64, i64)> = sees.get(&max_spot).unwrap().keys().map(|x| *x).collect();
    dir_vec.sort_by(|a, b| {
        let (x1, y1) = a;
        let (x2, y2) = b;
        let angle1 = heading(*x1, *y1);
        let angle2 = heading(*x2, *y2);
        return angle1.partial_cmp(&angle2).unwrap();
    });

    let mut dir_order: VecDeque<(i64, i64)> = dir_vec.iter().map(|x| *x).collect();

    let mut direction_vecs: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::from_iter(
        sees.get(&max_spot)
            .unwrap()
            .iter()
            .map(|(k, v)| (*k, v.iter().map(|x| *x).collect())),
    );
    for (_, v) in direction_vecs.iter_mut() {
        v.sort_by(|a, b| {
            let (x2, y2) = b;
            let (x1, y1) = a;
            let d1 = -manhattan_distance(max_spot.0, max_spot.1, *x1, *y1);
            let d2 = -manhattan_distance(max_spot.0, max_spot.1, *x2, *y2);
            return d1.partial_cmp(&d2).unwrap();
        });
    }

    for i in 0..200 {
        if dir_order.is_empty() {
            break;
        }

        while let Some(dir) = dir_order.front() {
            if direction_vecs.get(dir).unwrap().is_empty() {
                dir_order.pop_front();
            } else {
                break;
            }
        }

        let dir = dir_order.pop_front().unwrap();

        let (x, y) = direction_vecs.get_mut(&dir).unwrap().pop().unwrap();
        if i == 199 {
            return x * 100 + y;
        }

        dir_order.push_back(dir);
    }

    return -1;
}

fn euclids_extended_algorithm(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    } else {
        let (d, x, y) = euclids_extended_algorithm(b, a % b);
        return (d, y, x - y * (a / b));
    }
}

fn heading(x: i64, y: i64) -> f64 {
    let angle = (x as f64).atan2(-y as f64).to_degrees();
    let heading = if angle < 0.0 { angle + 360.0 } else { angle };

    return heading;
}

fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    return (x1 - x2).abs() + (y1 - y2).abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 210);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 802);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "10".to_string();

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
        "\nPart 1:\nMost asteroids seen: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLocation of 200th asteroid vaporized: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}