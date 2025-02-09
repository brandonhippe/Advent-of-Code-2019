use intcode_rust::Intcode;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut instructions: Intcode = Intcode::new(
        contents
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
    );
    instructions.insert(1, 12);
    instructions.insert(2, 2);
    if !instructions.run() {
        panic!("Program did not halt");
    }

    return *instructions.get(0).unwrap();
}

fn part2(contents: String) -> i64 {
    let instructions: Intcode = Intcode::new(
        contents
            .split(",")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
    );
    for n1 in 0..=99 {
        for n2 in 0..=99 {
            let mut temp_instructions = instructions.clone();
            temp_instructions.insert(1, n1);
            temp_instructions.insert(2, n2);
            if !temp_instructions.run() {

                panic!("Program did not halt");
            }

            if *temp_instructions.get(0).unwrap() == 19690720 {
                return 100 * n1 + n2;
            }
        }
    }

    return -1;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "2".to_string();

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
        "\nPart 1:\nResult after running program: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n100 * noun + verb: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}