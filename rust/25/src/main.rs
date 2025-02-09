use intcode_rust::Intcode;
use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let game = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());

    // DFS Traversal to get rooms
    let mut room_connections: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut program = game.clone();
    let mut stack: VecDeque<String> = VecDeque::new();

    let mut from_dir: String = "".to_string();
    let mut from_room: String = "".to_string();

    let room_regex: Regex = Regex::new(r"== (.+) ==\n").unwrap();
    let door_regex: Regex = Regex::new(r"Doors here lead:\n(-\s\w*\n)+").unwrap();
    let item_regex: Regex = Regex::new(r"Items here:\n(-.*\n)+").unwrap();

    let mut original_items: HashMap<String, String> = HashMap::new();
    let mut checkpoint: (String, String) = ("".to_string(), "".to_string());
    program.run();

    loop {
        let output = program
            .get_output()
            .iter()
            .map(|x| *x as u8 as char)
            .collect::<String>();
        program.clear_output();


        let mut curr_room: String = room_regex.captures(&output).unwrap()[1].to_string();
        let mut doors: Vec<String> = door_regex.captures(&output).unwrap()[0]
            .split("\n")
            .map(|x| x.to_string())
            .filter(|line| line.starts_with("-"))
            .map(|line| format!("{}\n", line[2..line.len()].to_string()))
            .collect();

        for cap in item_regex.captures_iter(&output) {
            let item: String = cap[0]
                .split("\n")
                .map(|x| x.to_string())
                .filter(|line| line.starts_with("-"))
                .map(|line| line[2..line.len()].to_string())
                .collect();
            original_items.insert(item, curr_room.clone());
        }

        if from_dir != "" {
            room_connections
                .entry(from_room.clone())
                .or_insert(HashMap::new())
                .insert(curr_room.clone(), from_dir.clone());
        }

        if output.contains("ejected back to the checkpoint") {
            checkpoint = (from_room.clone(), from_dir.clone());
            curr_room = from_room.clone();
        } else if !room_connections.contains_key(&curr_room) {
            while doors.len() != 1
                && match from_dir.as_str() {
                    "north\n" => doors[0] != "south\n",
                    "south\n" => doors[0] != "north\n",
                    "east\n" => doors[0] != "west\n",
                    "west\n" => doors[0] != "east\n",
                    _ => false,
                }
            {
                let removed = doors.remove(0);
                doors.push(removed);
            }

            stack.extend(doors);
        }

        if stack.is_empty() {
            break;
        }

        from_dir = stack.pop_back().unwrap();
        from_room = curr_room.clone();

        for c in from_dir.chars() {
            program.set_input(c as i64);
            program.run();
        }
    }

    // Floyd Warshall to get paths between rooms
    for intermediate in room_connections.clone().keys() {
        for start in room_connections.clone().keys() {
            for end in room_connections.clone().keys() {
                if room_connections
                    .get(start)
                    .unwrap()
                    .contains_key(intermediate)
                    && room_connections
                        .get(intermediate)
                        .unwrap()
                        .contains_key(end)
                {
                    let start_to_intermediate = room_connections
                        .get(start)
                        .unwrap()
                        .get(intermediate)
                        .unwrap()
                        .clone();
                    let intermediate_to_end = room_connections
                        .get(intermediate)
                        .unwrap()
                        .get(end)
                        .unwrap()
                        .clone();
                    let new_path = format!("{}{}", start_to_intermediate, intermediate_to_end);

                    if room_connections.get(start).unwrap().contains_key(end) {
                        let old_path_len: i64 = room_connections
                            .get(start)
                            .unwrap()
                            .get(end)
                            .unwrap()
                            .clone()
                            .chars()
                            .filter(|c| *c == '\n')
                            .count() as i64;
                        if (new_path.chars().filter(|c| *c == '\n').count() as i64) < old_path_len {
                            room_connections
                                .get_mut(start)
                                .unwrap()
                                .insert(end.clone(), new_path);
                        }
                    } else {
                        room_connections
                            .entry(start.clone())
                            .or_insert(HashMap::new())
                            .insert(end.clone(), new_path);
                    }
                }
            }
        }
    }

    let known_dangerous: Vec<&str> = vec![
        "giant electromagnet",
        "infinite loop",
    ];
    
    for item in known_dangerous {
        original_items.remove(item);
    }

    // Pick up items to see which are useful
    for (item, room) in original_items.clone() {
        let mut program = game.clone();
        for dir in room_connections
            .get("Hull Breach")
            .unwrap()
            .get(&room)
            .unwrap()
            .chars()
        {
            program.set_input(dir as i64);
            program.run();
        }

        program.clear_output();

        let mut okay = true;
        for c in format!("take {}\n", item).chars() {
            program.set_input(c as i64);
            okay = !program.run();
        }

        if !okay {
            original_items.remove(&item);
        }
    }

    // Get all the items and move to the checkpoint
    let mut program = game.clone();
    let mut curr_room: String = "Hull Breach".to_string();
    for (item, end_room) in original_items.iter() {
        for dir in room_connections
            .get(&curr_room)
            .unwrap()
            .get(end_room)
            .unwrap()
            .chars()
        {
            program.set_input(dir as i64);
            program.run();
        }

        program.clear_output();

        for c in format!("take {}\n", item).chars() {
            program.set_input(c as i64);
            program.run();
        }

        program.clear_output();
        curr_room = end_room.clone();
    }

    for dir in room_connections
        .get(&curr_room)
        .unwrap()
        .get(&checkpoint.0)
        .unwrap()
        .chars()
    {
        program.set_input(dir as i64);
        program.run();
    }

    for item in original_items.keys() {
        for c in format!("drop {}\n", item).chars() {
            program.set_input(c as i64);
            program.run();
        }
    }

    let item_mapping: HashMap<String, i64> = original_items
        .keys()
        .enumerate()
        .map(|(i, item)| (item.clone(), 1 << i as i64))
        .collect();

    for num in 0..(1 << original_items.len()) {
        program.clear_output();
        let take_items: Vec<String> = item_mapping
            .iter()
            .filter(|(_, mask)| num & **mask == **mask)
            .map(|(item, _)| item.clone())
            .collect();
        for item in take_items.iter() {
            for c in format!("take {}\n", item).chars() {
                program.set_input(c as i64);
                program.run();
            }
        }

        let mut found: bool = false;
        for dir in checkpoint.1.chars() {
            program.set_input(dir as i64);
            found = program.run();
        }

        if found {
            break;
        }

        for item in take_items.iter() {
            for c in format!("drop {}\n", item).chars() {
                program.set_input(c as i64);
                program.run();
            }
        }
    }

    // println!("{}", program.get_output().iter().map(|x| *x as u8 as char).collect::<String>());

    let int_re: Regex = Regex::new(r"(\d+)").unwrap();
    return int_re
        .captures(
            &program
                .get_output()
                .iter()
                .map(|x| *x as u8 as char)
                .collect::<String>(),
        )
        .unwrap()[1]
        .parse()
        .unwrap();
}

fn part2(_contents: String) -> String {
    return "Christmas has been saved!".to_string();
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "25".to_string();

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
        "\nPart 1:\nPassword for the main airlock: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n{}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}