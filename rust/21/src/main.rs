use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use intcode_rust::Intcode;

fn part1(contents: String) -> i64 {
    let mut program = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());
    let input_string = "NOT B J\nNOT C T\nOR T J\nAND D J\nNOT A T\nOR T J\nWALK\n".to_string();

    for c in input_string.chars() {
        program.run();
        program.set_input(c as i64);
    }

    if program.run() {
        let output = program.get_output();
        if output.last().unwrap() > &127 {
            return *output.last().unwrap();
        }
        
        for c in output {
            print!("{}", c as u8 as char);
        }
    } else {
        panic!("Program did not finish");
    }

    return -1;
}

fn part2(contents: String) -> i64 {
    let mut program = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());
    let input_string = "NOT B J\nNOT C T\nOR T J\nAND D J\nAND H J\nNOT A T\nOR T J\nRUN\n".to_string();


    for c in input_string.chars() {
        program.run();
        program.set_input(c as i64);
    }

    if program.run() {
        let output = program.get_output();
        if output.last().unwrap() > &127 {
            return *output.last().unwrap();
        }
        
        for c in output {
            print!("{}", c as u8 as char);
        }
    } else {
        panic!("Program did not finish");
    }

    return -1;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "21".to_string();

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
        "\nPart 1:\nHull Damage: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nHull Damage: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}