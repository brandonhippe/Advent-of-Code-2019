use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use intcode_rust::Intcode;
use std::collections::HashSet;

fn part1(contents: String) -> i64 {
    let mut program = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());
    program.run();

    let mut area: HashSet<(i64, i64)> = HashSet::new();
    for y in 0..50 {
        for x in 0..50 {
            let mut intcode = program.clone();
            intcode.set_input(x);
            intcode.run();
            intcode.set_input(y);
            intcode.run();

            let output = intcode.get_output();
            if output[0] == 1 {
                area.insert((x, y));
            }
        }
    }

    return area.len() as i64;
}

fn part2(contents: String) -> i64 {
    let mut program = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());
    program.run();

    let mut c = 1;

    let mut x = 0;
    let mut y = 0;
    while x == 0 && y == 0 {
        for y_test in 0..c {
            for x_test in 0..c {
                let mut intcode = program.clone();
                intcode.set_input(x_test);
                intcode.run();
                intcode.set_input(y_test);
                intcode.run();
                
                let output = intcode.get_output()[0];
                if output == 1 {
                    x = x_test;
                    y = y_test;
                    break;
                }
            }

            if x != 0 && y != 0 {
                break;
            }
        }

        c += 1;
    }

    loop {
        let mut intcode = program.clone();
        intcode.set_input(x);
        intcode.run();
        intcode.set_input(y);
        intcode.run();

        let mut output = intcode.get_output()[0];
        if output == 0 {
            x += 1;
            continue;
        }

        intcode = program.clone();
        intcode.set_input(x + 99);
        intcode.run();
        intcode.set_input(y - 99);
        intcode.run();

        output = intcode.get_output()[0];
        if output == 0 {
            y += 1;
            continue;
        }

        return x * 10000 + y - 99;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "19".to_string();

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
        "\nPart 1:\nPoints in beam in 50x50 area: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nCoordinate of top left corner of first 100x100 area: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}