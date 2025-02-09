use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use intcode_rust::Intcode;
use std::collections::HashMap;
use std::collections::HashSet;

fn part1(contents: String) -> i64 {
    let mut painted: HashSet<(i64, i64)> = HashSet::new();
    let mut colors: HashMap<(i64, i64), i64> = HashMap::new();
    let mut robot = Intcode::new(
        contents
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
    );
    let mut pos: (i64, i64) = (0, 0);
    let mut dir: (i64, i64) = (0, 1);

    loop {
        robot.set_input(*colors.get(&pos).unwrap_or(&0));
        let halted = robot.run();
        let outputs = robot.get_output();
        robot.clear_output();

        colors.insert(pos, outputs[outputs.len() - 2]);
        painted.insert(pos);

        if outputs[outputs.len() - 1] == 0 {
            dir = (-dir.1, dir.0);
        } else {
            dir = (dir.1, -dir.0);
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);


        if halted {
            break;
        }
    }

    return painted.len() as i64;
}

fn part2(contents: String) -> String {
    let mut colors: HashMap<(i64, i64), i64> = HashMap::new();
    let mut robot = Intcode::new(
        contents
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
    );
    let mut pos: (i64, i64) = (0, 0);
    let mut dir: (i64, i64) = (0, 1);
    colors.insert(pos, 1);

    loop {
        robot.set_input(*colors.get(&pos).unwrap_or(&0));
        let halted = robot.run();
        let outputs = robot.get_output();
        robot.clear_output();

        colors.insert(pos, outputs[outputs.len() - 2]);

        if outputs[outputs.len() - 1] == 0 {
            dir = (-dir.1, dir.0);
        } else {
            dir = (dir.1, -dir.0);
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);

        if halted {
            break;
        }
    }

    let min_x = colors.keys().map(|x| x.0).min().unwrap();
    let max_x = colors.keys().map(|x| x.0).max().unwrap();
    let min_y = colors.keys().map(|x| x.1).min().unwrap();
    let max_y = colors.keys().map(|x| x.1).max().unwrap();

    let mut output = String::new();

    for y in (min_y..=max_y).rev() {
        output.push_str("\n");
        for x in min_x..=max_x {
            if *colors.get(&(x, y)).unwrap_or(&0) == 1 {
                output.push_str("█");
            } else {
                output.push_str(" ");
            }
        }
    }

    return output;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "11".to_string();

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
        "\nPart 1:\nNumber of tiles painted at least once: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRegistration Identifier: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}