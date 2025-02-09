use intcode_rust::Intcode;
use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut intcode = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());
    intcode.run();

    let view: String = intcode
        .get_output()
        .iter()
        .map(|x| *x as u8 as char)
        .collect();
    return alignment_params(view);
}

fn part2(contents: String) -> i64 {
    let mut intcode = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());
    intcode.run();

    let view: String = intcode
        .get_output()
        .iter()
        .map(|x| *x as u8 as char)
        .collect();
    let (m, a, b, c) = instruction_sequence(view);
    let m_set: HashSet<char> = HashSet::from_iter(m.chars());

    if m_set != HashSet::from_iter(vec!['A', 'B', 'C', ','].iter().map(|x| *x)) {
        panic!("Instruction sequence failed");
    }


    let input_string = format!("{}\n{}\n{}\n{}\nn\n", m, a, b, c);
    intcode = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());
    intcode.insert(0, 2);

    for c in input_string.chars() {
        intcode.set_input(c as i64);
        intcode.run();
    }

    return intcode.get_output().pop().unwrap();
}

fn alignment_params(contents: String) -> i64 {
    let mut sum: i64 = 0;
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            area.insert((x as i64, y as i64));
        }
    }

    for (x, y) in area.iter() {
        if area.contains(&(*x + 1, *y))
            && area.contains(&(*x - 1, *y))
            && area.contains(&(*x, *y + 1))
            && area.contains(&(*x, *y - 1))
        {
            sum += x * y;
        }
    }

    return sum;
}

fn instruction_sequence(contents: String) -> (String, String, String, String) {
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    let mut robot: (i64, i64) = (0, 0);
    let mut robot_dir: (i64, i64) = (0, 0);

    for (y, line) in contents.lines().enumerate() {
        for (x, l) in line.chars().enumerate() {
            match l {
                '^' => {
                    robot = (x as i64, y as i64);
                    robot_dir = (0, -1);
                }
                'v' => {
                    robot = (x as i64, y as i64);
                    robot_dir = (0, 1);
                }
                '<' => {
                    robot = (x as i64, y as i64);
                    robot_dir = (-1, 0);
                }
                '>' => {
                    robot = (x as i64, y as i64);
                    robot_dir = (1, 0);
                }
                _ => {}
            }

            if l != '.' {
                area.insert((x as i64, y as i64));
            }
        }
    }

    let mut move_str = "".to_string();
    let mut move_amt = 0;
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    while visited.len() != area.len() {
        visited.insert(robot);
        if area.contains(&(robot.0 + robot_dir.0, robot.1 + robot_dir.1)) {
            robot = (robot.0 + robot_dir.0, robot.1 + robot_dir.1);
            move_amt += 1;
        } else if area.contains(&(robot.0 + robot_dir.1, robot.1 - robot_dir.0)) {
            move_str = format!("{},{},L", move_str, move_amt);
            move_amt = 0;
            robot_dir = (robot_dir.1, -robot_dir.0);
        } else if area.contains(&(robot.0 - robot_dir.1, robot.1 + robot_dir.0)) {
            move_str = format!("{},{},R", move_str, move_amt);
            move_amt = 0;
            robot_dir = (-robot_dir.1, robot_dir.0);
        }
    }

    move_str = format!("{},{}", &move_str[3..], move_amt);

    let a_str = find_group(&mut move_str, "A");
    let b_str = find_group(&mut move_str, "B");
    let c_str = find_group(&mut move_str, "C");

    return (move_str, a_str, b_str, c_str);
}

fn find_group(move_str: &mut String, replace_with: &str) -> String {
    let start_ix = move_str
        .chars()
        .enumerate()
        .filter(|(_, c)| *c != ',' && *c != 'A' && *c != 'B' && *c != 'C')
        .map(|(i, _)| i)
        .min()
        .unwrap();
    let end_ix = start_ix + 20;

    for ix in move_str
        .clone()
        .match_indices(",")
        .filter(|(i, _)| *i >= start_ix && *i < end_ix)
        .collect::<Vec<_>>()
        .iter()
        .rev()
    {
        let (i, _) = ix;
        let potential_move = &move_str[start_ix..*i].to_string();
        if potential_move
            .chars()
            .nth(potential_move.len() - 1)
            .unwrap()
            .is_alphabetic()
        {
            continue;
        }

        if move_str.matches(potential_move).count() > 1 {
            *move_str = move_str.replace(potential_move, replace_with);
            return potential_move.to_string();
        }
    }

    panic!("No group found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(alignment_params(contents), 76);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "17".to_string();

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
        "\nPart 1:\nSum of alignment parameters: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nDust Collected: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}