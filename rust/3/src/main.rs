use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut total: i64 = 0;
    let mut sets: Vec<HashSet<(i64, i64)>> = vec![];

    for line in contents.lines() {
        let mut set = HashSet::new();
        let mut pos = (0, 0);

        for ins in line.split(",") {
            let dir = ins.chars().nth(0).unwrap();
            let dist = ins[1..].parse::<i64>().unwrap();
            for _ in 0..dist {
                match dir {
                    'U' => pos.1 += 1,
                    'D' => pos.1 -= 1,
                    'L' => pos.0 -= 1,
                    'R' => pos.0 += 1,
                    _ => panic!("Invalid direction"),
                }
                set.insert(pos);
            }
        }

        sets.push(set);
    }

    let mut intersections = sets[0].clone();

    for set in sets {
        intersections = intersections.intersection(&set).map(|x| *x).collect();
    }

    total += intersections
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap();

    return total;
}

fn part2(contents: String) -> i64 {
    let mut total: i64 = 0;
    let mut dicts: Vec<HashMap<(i64, i64), i64>> = vec![];

    for line in contents.lines() {
        let mut dict = HashMap::new();
        let mut pos = (0, 0);
        let mut steps = 0;

        for ins in line.split(",") {
            let dir = ins.chars().nth(0).unwrap();
            let dist = ins[1..].parse::<i64>().unwrap();
            for _ in 0..dist {
                match dir {
                    'U' => pos.1 += 1,
                    'D' => pos.1 -= 1,
                    'L' => pos.0 -= 1,
                    'R' => pos.0 += 1,
                    _ => panic!("Invalid direction"),
                }

                steps += 1;
                dict.insert(pos, steps);
            }
        }

        dicts.push(dict);
    }

    let mut intersections = dicts[0].keys().cloned().collect::<HashSet<(i64, i64)>>();
    for dict in &dicts {
        intersections = intersections
            .intersection(&dict.keys().cloned().collect::<HashSet<(i64, i64)>>())
            .map(|x| *x)
            .collect();
    }

    total += intersections
        .iter()
        .map(|pos| dicts.iter().map(|dict| dict.get(pos).unwrap()).sum::<i64>())
        .min()
        .unwrap();

    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 300);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 1050);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "3".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("rust_{}_{}", year, day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };


    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nManhattan distance to closest itersection: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of steps to closest intersection: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}