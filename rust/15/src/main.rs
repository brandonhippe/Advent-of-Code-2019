use cached::proc_macro::cached;
use intcode_rust::Intcode;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return area(contents).0;
}

fn part2(contents: String) -> i64 {
    return area(contents).1;
}

#[cached]
fn area(contents: String) -> (i64, i64) {
    let mut area: HashMap<(i64, i64), i64> = HashMap::new();
    let mut pos = (0, 0);

    let mut intcode = Intcode::new(
        contents
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect(),
    );

    if intcode.run() {
        panic!("Shouldn't have halted");
    }

    let directions = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut stack: Vec<(i64, i64)> = vec![(1, 0)];

    let mut found_oxygen: bool = false;
    let mut oxygen: (i64, i64) = (0, 0);

    while stack.len() > 0 {
        area.insert(pos, if pos == oxygen { 0 } else { i64::MAX });
        let (dir, back) = stack.pop().unwrap();

        if dir > 4 {
            if back == 0 {
                continue;
            }

            intcode.set_input(back);
            if intcode.run() {
                panic!("Shouldn't have halted");
            }

            let output = intcode.get_output();
            if output.len() > 1 {
                panic!("Output len was greater than 1");
            }

            if output[0] == 0 {
                panic!("Wasn't able to go back");
            }

            pos = (
                pos.0 + directions[(back - 1) as usize].0,
                pos.1 + directions[(back - 1) as usize].1,
            );

            intcode.clear_output();
        } else {
            stack.push((dir + 1, back));
            if dir == back {
                continue;
            }

            let new_pos = (
                pos.0 + directions[(dir - 1) as usize].0,
                pos.1 + directions[(dir - 1) as usize].1,
            );

            if area.contains_key(&new_pos) {
                continue;
            }

            intcode.set_input(dir);

            if intcode.run() {
                panic!("Shouldn't have halted");
            }

            let output = intcode.get_output();
            if output.len() > 1 {
                panic!("Output len was greater than 1");
            }

            intcode.clear_output();

            if output[0] == 0 {
                continue;
            }

            pos = new_pos;
            stack.push((
                1,
                match dir {
                    1 => 2,
                    2 => 1,
                    3 => 4,
                    4 => 3,
                    _ => 0,
                },
            ));

            if output[0] == 2 {
                if found_oxygen {
                    if pos != oxygen {
                        panic!("Found multiple oxygens");
                    }
                } else {
                    found_oxygen = true;
                    oxygen = pos;
                }
            }
        }
    }

    if !found_oxygen {
        panic!("Didn't find oxygen");
    }

    let mut open_list: VecDeque<((i64, i64), i64)> = VecDeque::new();
    open_list.push_back((oxygen, 0));

    while let Some((pos, d)) = open_list.pop_front() {
        area.insert(pos, d);

        for offset in directions.iter() {
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

            if area.contains_key(&new_pos) && *area.get(&new_pos).unwrap() > d + 1 {
                open_list.push_back((new_pos, d + 1));
            }
        }
    }

    return (*area.get(&(0, 0)).unwrap(), *area.values().max().unwrap());
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "15".to_string();

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
        "\nPart 1:\nDistance to oxygen: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTime to fill: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}