use intcode_rust::Intcode;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut intcode = Intcode::new(
        contents
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect(),
    );

    let mut screen: HashMap<(i64, i64), i64> = HashMap::new();
    if !intcode.run() {
        panic!("Intcode should have halted");
    }

    let outputs = intcode.get_output().clone();
    for i in 0..outputs.len() / 3 {
        let x = outputs[i * 3];
        let y = outputs[i * 3 + 1];
        let tile = outputs[i * 3 + 2];
        screen.insert((x, y), tile);
    }

    return screen.values().filter(|x| **x == 2).count() as i64;
}

fn part2(contents: String) -> i64 {
    let mut intcode = Intcode::new(
        contents
            .split(',')

            .map(|x| x.parse::<i64>().unwrap())
            .collect(),
    );

    intcode.insert(0, 2);

    let mut screen: HashMap<(i64, i64), i64> = HashMap::new();
    while !intcode.run() {
        handle(&mut intcode, &mut screen);
    }

    return handle(&mut intcode, &mut screen);
}

fn handle(intcode: &mut Intcode, screen: &mut HashMap<(i64, i64), i64>) -> i64 {
    let outputs = intcode.get_output().clone();
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;

    for i in 0..outputs.len() / 3 {
        let x = outputs[i * 3];
        let y = outputs[i * 3 + 1];
        let tile = outputs[i * 3 + 2];

        if x == -1 && y == 0 {
            score = tile;
        } else {
            screen.insert((x, y), tile);
        }

        if tile == 3 {
            paddle_x = x;
        } else if tile == 4 {
            ball_x = x;
        }
    }

    intcode.set_input((ball_x - paddle_x).signum());
    intcode.clear_output();

    return score;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "13".to_string();

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
        "\nPart 1:\nNumber of block tiles: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nScore after beating game: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}