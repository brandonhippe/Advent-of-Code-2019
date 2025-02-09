use relative_path::RelativePath;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    let mut portals: HashMap<String, Vec<(i64, i64)>> = HashMap::new();
    let mut portal_areas: HashMap<(i64, i64), String> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    area.insert((x as i64, y as i64));
                }
                _ => {}
            }

            if c.is_alphabetic() && x != 0 && y != 0 && x != line.len() - 1 && y != lines.len() - 1
            {
                let deltas: Vec<(i64, i64)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
                for (dx, dy) in deltas.iter() {
                    if lines[(y as i64 + dy) as usize]
                        .chars()
                        .nth((x as i64 + dx) as usize)
                        .unwrap()
                        == '.'
                    {
                        let portal_name = if *dx == 1 || *dy == 1 {
                            format!(

                                "{}{}",
                                lines[(y as i64 - dy) as usize]
                                    .chars()
                                    .nth((x as i64 - dx) as usize)
                                    .unwrap(),
                                c
                            )
                        } else {
                            format!(
                                "{}{}",
                                c,
                                lines[(y as i64 - dy) as usize]
                                    .chars()
                                    .nth((x as i64 - dx) as usize)
                                    .unwrap()
                            )
                        };

                        if !portals.contains_key(&portal_name) {
                            portals.insert(portal_name.clone(), Vec::new());
                        }

                        portals
                            .get_mut(&portal_name)
                            .unwrap()
                            .push((x as i64 + dx, y as i64 + dy));
                        portal_areas.insert((x as i64, y as i64), portal_name);
                    }
                }
            }
        }
    }

    let mut connections: HashMap<(i64, i64), HashMap<(i64, i64), i64>> = HashMap::from_iter(
        area.iter()
            .filter(|(x, y)| {
                let mut neighbors: i64 = 0;
                for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    if area.contains(&(x + dx, y + dy)) {
                        neighbors += 1;
                    }

                    if portal_areas.contains_key(&(x + dx, y + dy)) {
                        neighbors += 1;
                    }
                }

                neighbors > 2
            })
            .map(|(x, y)| ((*x, *y), HashMap::new())),
    );

    let start = portals.get("AA").unwrap()[0];
    let end = portals.get("ZZ").unwrap()[0];
    connections.insert(start, HashMap::new());
    connections.insert(end, HashMap::new());

    for start_pos in connections.clone().keys() {
        let mut open_list: VecDeque<((i64, i64), i64)> = VecDeque::new();
        let mut visited: HashSet<(i64, i64)> = HashSet::new();
        open_list.push_back((*start_pos, 0));

        while let Some((pos, d)) = open_list.pop_front() {
            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);

            if connections.contains_key(&pos) && pos != *start_pos {
                connections.get_mut(&start_pos).unwrap().insert(pos, d);
                continue;
            }

            for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_pos = (pos.0 + dx, pos.1 + dy);
                if visited.contains(&new_pos) {
                    continue;
                }

                if area.contains(&new_pos) {
                    open_list.push_back((new_pos, d + 1));
                }

                if pos != start && pos != end && portal_areas.contains_key(&new_pos) {
                    let portal_name = portal_areas.get(&new_pos).unwrap();
                    let other_portal = portals
                        .get(portal_name)
                        .unwrap()
                        .iter()
                        .find(|p| **p != pos)
                        .unwrap();
                    open_list.push_back((*other_portal, d + 1));
                }
            }
        }
    }

    let mut open_list: BinaryHeap<(i64, (i64, i64))> = BinaryHeap::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    open_list.push((0, start));

    while let Some((d, pos)) = open_list.pop() {
        visited.insert(pos);

        if pos == end {
            return -d;
        }

        for (n, dist) in connections.get(&pos).unwrap() {
            if !visited.contains(n) {
                open_list.push((d - dist, *n));
            }
        }
    }

    return -1;
}

fn part2(contents: String) -> i64 {
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    let mut portals: HashMap<String, Vec<(i64, i64, i64)>> = HashMap::new();
    let mut portal_areas: HashMap<(i64, i64), String> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    area.insert((x as i64, y as i64));
                }
                _ => {}
            }

            if c.is_alphabetic() && x != 0 && y != 0 && x != line.len() - 1 && y != lines.len() - 1
            {
                let deltas: Vec<(i64, i64)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
                for (dx, dy) in deltas.iter() {
                    if lines[(y as i64 + dy) as usize]
                        .chars()
                        .nth((x as i64 + dx) as usize)
                        .unwrap()
                        == '.'
                    {
                        let portal_name = if *dx == 1 || *dy == 1 {
                            format!(
                                "{}{}",
                                lines[(y as i64 - dy) as usize]
                                    .chars()
                                    .nth((x as i64 - dx) as usize)
                                    .unwrap(),
                                c
                            )
                        } else {
                            format!(
                                "{}{}",
                                c,
                                lines[(y as i64 - dy) as usize]
                                    .chars()
                                    .nth((x as i64 - dx) as usize)
                                    .unwrap()
                            )
                        };

                        if !portals.contains_key(&portal_name) {
                            portals.insert(portal_name.clone(), Vec::new());
                        }

                        let outside =
                            x == 1 || y == 1 || x == (line.len() - 2) || y == (lines.len() - 2);

                        portals.get_mut(&portal_name).unwrap().push((
                            x as i64 + dx,
                            y as i64 + dy,
                            if outside { -1 } else { 1 },
                        ));
                        portal_areas.insert((x as i64, y as i64), portal_name);
                    }
                }
            }
        }
    }

    let mut connections: HashMap<(i64, i64), HashMap<(i64, i64), (i64, i64)>> = HashMap::from_iter(
        area.iter()
            .filter(|(x, y)| {
                let mut neighbors: i64 = 0;
                for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    if area.contains(&(x + dx, y + dy)) {
                        neighbors += 1;
                    }

                    if portal_areas.contains_key(&(x + dx, y + dy)) {
                        neighbors += 1;
                    }
                }

                neighbors > 2
            })
            .map(|(x, y)| ((*x, *y), HashMap::new())),
    );

    let start = (
        portals.get("AA").unwrap()[0].0,
        portals.get("AA").unwrap()[0].1,
    );
    let end = (
        portals.get("ZZ").unwrap()[0].0,
        portals.get("ZZ").unwrap()[0].1,
    );

    for pos in portals.keys() {
        if pos == "AA" || pos == "ZZ" {
            continue;
        }

        let portal = portals.get(pos).unwrap();
        let first = portal[0];
        let second = portal[1];
        let first_pos = (first.0, first.1);
        let second_pos = (second.0, second.1);

        connections.insert(first_pos, HashMap::new());
        connections.insert(second_pos, HashMap::new());

        connections
            .get_mut(&first_pos)
            .unwrap()
            .insert(second_pos, (1, first.2));
        connections
            .get_mut(&second_pos)
            .unwrap()
            .insert(first_pos, (1, second.2));
    }

    connections.insert(start, HashMap::new());
    connections.insert(end, HashMap::new());

    for start_pos in connections.clone().keys() {
        let mut open_list: VecDeque<((i64, i64), i64)> = VecDeque::new();
        let mut visited: HashSet<(i64, i64)> = HashSet::new();
        open_list.push_back((*start_pos, 0));

        while let Some((pos, d)) = open_list.pop_front() {
            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);

            if connections.contains_key(&pos) && pos != *start_pos {
                connections.get_mut(&start_pos).unwrap().insert(pos, (d, 0));
                continue;
            }

            for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_pos = (pos.0 + dx, pos.1 + dy);
                if visited.contains(&new_pos) {
                    continue;
                }

                if area.contains(&new_pos) {
                    open_list.push_back((new_pos, d + 1));
                }
            }
        }
    }

    let mut open_list: BinaryHeap<(i64, (i64, i64, i64))> = BinaryHeap::new();
    let mut visited: HashSet<(i64, i64, i64)> = HashSet::new();
    open_list.push((0, (start.0, start.1, 0)));

    while let Some((d, (x, y, depth))) = open_list.pop() {
        visited.insert((x, y, depth));

        if (x, y, depth) == (end.0, end.1, 0) {
            return -d;
        }

        for ((nx, ny), (dist, d_depth)) in connections.get(&(x, y)).unwrap() {
            if depth + d_depth < 0 {
                continue;
            }

            if depth != 0 && ((*nx, *ny) == start || (*nx, *ny) == end) {
                continue;
            }

            if !visited.contains(&(*nx, *ny, depth + d_depth)) {
                open_list.push((d - dist, (*nx, *ny, depth + d_depth)));
            }
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 58);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 396);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "20".to_string();

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
        "\nPart 1:\nShortest path from AA to ZZ: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nShortest path from AA to ZZ: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}