use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1<'a>(contents: String) -> i64 {
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for line in contents.lines() {
        let mut split = line.split(")");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        orbits.insert(b, a);
    }

    let mut memo: HashMap<&str, i64> = HashMap::new();
    memo.insert("COM", 0);
    for (k, _) in orbits.clone() {
        count_orbits(orbits.clone(), k, &mut memo);
    }

    return memo.values().sum();
}

fn part2(contents: String) -> i64 {
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for line in contents.lines() {
        let mut split = line.split(")");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        orbits.insert(b, a);
    }


    let mut you_orbit: HashMap<&str, i64> = HashMap::new();
    let mut santa_orbit: HashMap<&str, i64> = HashMap::new();
    let mut you_pos = orbits.get("YOU").unwrap();
    let mut santa_pos = orbits.get("SAN").unwrap();
    let mut steps = 0;

    loop {
        if !you_orbit.contains_key(you_pos) {
            you_orbit.insert(you_pos, steps);
        }

        if !santa_orbit.contains_key(santa_pos) {
            santa_orbit.insert(santa_pos, steps);
        }

        if you_orbit.contains_key(santa_pos) {
            return you_orbit.get(santa_pos).unwrap() + santa_orbit.get(santa_pos).unwrap();
        } else if santa_orbit.contains_key(you_pos) {
            return you_orbit.get(you_pos).unwrap() + santa_orbit.get(you_pos).unwrap();
        }

        you_pos = orbits.get(you_pos).unwrap_or(you_pos);
        santa_pos = orbits.get(santa_pos).unwrap_or(santa_pos);
        steps += 1;
    }
}

fn count_orbits<'a>(
    orbits: HashMap<&str, &'a str>,
    key: &'a str,
    memo: &mut HashMap<&'a str, i64>,
) -> i64 {
    if memo.contains_key(key) {
        return *memo.get(key).unwrap();
    }

    let count = 1 + count_orbits(orbits.clone(), orbits.get(key).unwrap(), memo);

    memo.insert(key, count);
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 42);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 4);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "6".to_string();

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
        "\nPart 1:\nTotal number of orbits: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPath length to Santa: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}