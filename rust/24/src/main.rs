use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut area: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                area.insert((x as i64, y as i64));
            }
        }
    }

    let mut seen: HashSet<i64> = HashSet::new();
    while !seen.contains(&biodiversity(&area)) {
        seen.insert(biodiversity(&area));

        let mut new_area: HashSet<(i64, i64)> = HashSet::new();

        for y in 0..5 {
            for x in 0..5 {
                let mut neighbors = 0;

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                    if area.contains(&(x + dx, y + dy)) {
                        neighbors += 1;
                    }
                }

                if area.contains(&(x, y)) {

                    if neighbors == 1 {
                        new_area.insert((x, y));
                    }
                } else {
                    if neighbors == 1 || neighbors == 2 {
                        new_area.insert((x, y));
                    }
                }
            }
        }

        area = new_area;
    }

    return biodiversity(&area);

    fn biodiversity(area: &HashSet<(i64, i64)>) -> i64 {
        let mut result = 0;
        let mut power = 1;

        for y in 0..5 {
            for x in 0..5 {
                if area.contains(&(x, y)) {
                    result += power;
                }
                power <<= 1;
            }
        }

        return result;
    }
}

fn part2(contents: String, time: i64) -> i64 {
    let mut area: HashSet<(i64, i64, i64)> = HashSet::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                area.insert((0, x as i64, y as i64));
            }
        }
    }

    for _ in 0..time {
        let mut new_area: HashMap<(i64, i64, i64), i64> = HashMap::new();

        for (depth, x, y) in area.iter() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                if *x + dx == 2 && *y + dy == 2 {
                    // Inner
                    for i in 0..5 {
                        if *dx == 0 {
                            new_area
                                .entry((*depth + 1, i, if *dy == -1 { 4 } else { 0 }))
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                        } else {
                            new_area
                                .entry((*depth + 1, if *dx == -1 { 4 } else { 0 }, i))
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                        }
                    }
                } else if *x + dx < 0 {
                    // Outer
                    new_area
                        .entry((*depth - 1, 1, 2))
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                } else if *x + dx >= 5 {
                    // Outer
                    new_area
                        .entry((*depth - 1, 3, 2))
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                } else if *y + dy < 0 {
                    // Outer
                    new_area
                        .entry((*depth - 1, 2, 1))
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                } else if *y + dy >= 5 {
                    // Outer
                    new_area
                        .entry((*depth - 1, 2, 3))
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                } else {
                    new_area
                        .entry((*depth, *x + dx, *y + dy))
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
        }

        area = new_area
            .iter()
            .filter(|(k, &v)| {
                if area.contains(k) {
                    return v == 1;
                } else {
                    return v == 1 || v == 2;
                }
            })
            .map(|(k, _)| *k)
            .collect();
    }

    return area.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 2129920);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 10), 99);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "24".to_string();

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
        "\nPart 1:\nBiodiversity of first layout that appears twice: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nBugs present after 200 minutes: {}\nRan in {:.5?}",
        part2(contents.clone(), 200),
        part2_timer.elapsed()
    );
}