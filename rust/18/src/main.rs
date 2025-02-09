use relative_path::RelativePath;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut doors: HashMap<(i64, i64), char> = HashMap::new();
    let mut keys: HashMap<(i64, i64), char> = HashMap::new();
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    let mut start: (i64, i64) = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {}
                '.' => {
                    area.insert((x as i64, y as i64));
                }
                '@' => {
                    start = (x as i64, y as i64);
                    area.insert((x as i64, y as i64));
                }
                _ => {
                    if c.is_ascii_lowercase() {
                        keys.insert((x as i64, y as i64), c);
                        area.insert((x as i64, y as i64));
                    } else if c.is_ascii_uppercase() {
                        doors.insert((x as i64, y as i64), c);
                        area.insert((x as i64, y as i64));
                    }
                }
            }

        }
    }

    let mut neighbors: HashMap<u32, HashMap<u32, Neighbor>> =
        HashMap::from_iter(keys.iter().map(|(k, v)| {
            (
                *v as u32 - 'a' as u32 + 1,
                get_neighbors(*k, &area, &doors, &keys),
            )
        }));
    neighbors.insert(0, get_neighbors(start, &area, &doors, &keys));

    // Floyd-Warshall Algorithm
    for (k, n1) in neighbors.clone().iter() {
        for (i, n2) in n1.iter() {
            for (j, n3) in n1.iter() {
                if i == j {
                    continue;
                }
                let new_distance = n2.distance + n3.distance;
                if let Some(n4) = neighbors.get_mut(j) {
                    if let Some(n5) = n4.get_mut(&i) {
                        if new_distance < n5.distance {
                            n5.distance = new_distance;
                            n5.required = n2.required | n3.required;
                        }
                    } else {
                        n4.insert(
                            *i,
                            Neighbor {
                                distance: new_distance,
                                required: n2.required | n3.required,
                            },
                        );
                    }
                } else {
                    let mut new_neighbor = HashMap::new();
                    new_neighbor.insert(
                        *i,
                        Neighbor {
                            distance: new_distance,
                            required: n2.required | n3.required,
                        },
                    );
                    neighbors.insert(*j, new_neighbor);
                }
            }
        }
    }

    let goal = (1 << keys.len()) - 1;
    let mut heap: BinaryHeap<(i64, u32, u32)> = BinaryHeap::new();
    heap.push((0, 0, 0));
    let mut visited: HashSet<(u32, u32)> = HashSet::new();

    while !heap.is_empty() {
        let (distance, current, keys) = heap.pop().unwrap();
        if keys == goal {
            return -distance;
        }

        if visited.contains(&(current, keys)) {
            continue;
        }

        visited.insert((current, keys));
        for (neighbor, n) in neighbors.get(&current).unwrap() {
            if keys & n.required == n.required {
                heap.push((
                    distance - n.distance,
                    *neighbor,
                    keys | (1 << (*neighbor - 1)),
                ));
            }
        }
    }

    return -1;
}

fn part2(contents: String) -> i64 {
    let mut doors: HashMap<(i64, i64), char> = HashMap::new();
    let mut keys: HashMap<(i64, i64), char> = HashMap::new();
    let mut area: HashSet<(i64, i64)> = HashSet::new();
    let mut start: (i64, i64) = (0, 0);
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {}
                '.' => {
                    area.insert((x as i64, y as i64));
                }
                '@' => {
                    start = (x as i64, y as i64);
                    area.insert((x as i64, y as i64));
                }
                _ => {
                    if c.is_ascii_lowercase() {
                        keys.insert((x as i64, y as i64), c);
                        area.insert((x as i64, y as i64));
                    } else if c.is_ascii_uppercase() {
                        doors.insert((x as i64, y as i64), c);
                        area.insert((x as i64, y as i64));
                    }
                }
            }
        }
    }

    area = area
        .difference(&HashSet::from_iter(vec![
            (start.0, start.1),
            (start.0 + 1, start.1),
            (start.0 - 1, start.1),
            (start.0, start.1 + 1),
            (start.0, start.1 - 1),
        ]))
        .map(|p| *p)
        .collect();
    let starts = vec![
        (start.0 + 1, start.1 + 1),
        (start.0 - 1, start.1 + 1),
        (start.0 + 1, start.1 - 1),
        (start.0 - 1, start.1 - 1),
    ];

    let mut neighbors: HashMap<u32, HashMap<u32, Neighbor>> =
        HashMap::from_iter(keys.iter().map(|(k, v)| {
            (
                *v as u32 - 'a' as u32 + 1,
                get_neighbors(*k, &area, &doors, &keys),
            )
        }));
    for (i, s) in starts.iter().enumerate() {
        neighbors.insert(
            (i as u32) ^ 0xffffffff,
            get_neighbors(*s, &area, &doors, &keys),
        );
    }

    // Floyd-Warshall Algorithm
    for (k, n1) in neighbors.clone().iter() {
        for (i, n2) in n1.iter() {
            for (j, n3) in n1.iter() {
                if i == j {
                    continue;
                }
                let new_distance = n2.distance + n3.distance;
                if let Some(n4) = neighbors.get_mut(j) {
                    if let Some(n5) = n4.get_mut(&i) {
                        if new_distance < n5.distance {
                            n5.distance = new_distance;
                            n5.required = n2.required | n3.required;
                        }
                    } else {
                        n4.insert(
                            *i,
                            Neighbor {
                                distance: new_distance,
                                required: n2.required | n3.required,
                            },
                        );
                    }
                } else {
                    let mut new_neighbor = HashMap::new();
                    new_neighbor.insert(
                        *i,
                        Neighbor {
                            distance: new_distance,
                            required: n2.required | n3.required,
                        },
                    );
                    neighbors.insert(*j, new_neighbor);
                }
            }
        }
    }

    let goal = (1 << keys.len()) - 1;
    let mut heap: BinaryHeap<(i64, u32, u32, u32, u32, u32)> = BinaryHeap::new();
    let mut visited: HashSet<(u32, u32)> = HashSet::new();
    heap.push((
        0,
        0 ^ 0xffffffff,
        1 ^ 0xffffffff,
        2 ^ 0xffffffff,
        3 ^ 0xffffffff,
        0,
    ));

    while !heap.is_empty() {
        let (distance, c1, c2, c3, c4, keys) = heap.pop().unwrap();
        if keys == goal {
            return -distance;
        }

        if visited.contains(&(c1, keys))
            && visited.contains(&(c2, keys))
            && visited.contains(&(c3, keys))
            && visited.contains(&(c4, keys))
        {
            continue;
        }

        visited.insert((c1, keys));
        for (neighbor, n) in neighbors.get(&c1).unwrap() {
            if keys & n.required == n.required {
                heap.push((
                    distance - n.distance,
                    *neighbor,
                    c2,
                    c3,
                    c4,
                    keys | (1 << (*neighbor - 1)),
                ));
            }
        }

        visited.insert((c2, keys));
        for (neighbor, n) in neighbors.get(&c2).unwrap() {
            if keys & n.required == n.required {
                heap.push((
                    distance - n.distance,
                    c1,
                    *neighbor,
                    c3,
                    c4,
                    keys | (1 << (*neighbor - 1)),
                ));
            }
        }

        visited.insert((c3, keys));
        for (neighbor, n) in neighbors.get(&c3).unwrap() {
            if keys & n.required == n.required {
                heap.push((
                    distance - n.distance,
                    c1,
                    c2,
                    *neighbor,
                    c4,
                    keys | (1 << (*neighbor - 1)),
                ));
            }
        }

        visited.insert((c4, keys));
        for (neighbor, n) in neighbors.get(&c4).unwrap() {
            if keys & n.required == n.required {
                heap.push((
                    distance - n.distance,
                    c1,
                    c2,
                    c3,
                    *neighbor,
                    keys | (1 << (*neighbor - 1)),
                ));
            }
        }
    }

    return -1;
}

fn get_neighbors(
    key: (i64, i64),
    area: &HashSet<(i64, i64)>,
    doors: &HashMap<(i64, i64), char>,
    keys: &HashMap<(i64, i64), char>,
) -> HashMap<u32, Neighbor> {
    let mut neighbors: HashMap<u32, Neighbor> = HashMap::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut queue: VecDeque<(i64, i64, i64, HashSet<char>)> = VecDeque::new();
    queue.push_back((key.0, key.1, 0, HashSet::new()));
    visited.insert(key);

    while !queue.is_empty() {
        let (x, y, distance, keys_needed) = queue.pop_front().unwrap();
        for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = x + dx;
            let new_y = y + dy;
            let new_pos = (new_x, new_y);
            let mut new_keys_needed = keys_needed.clone();

            if visited.contains(&new_pos) || !area.contains(&new_pos) {
                continue;
            }

            visited.insert(new_pos);
            if let Some(key) = keys.get(&new_pos) {
                neighbors.insert(
                    *key as u32 - 'a' as u32 + 1,
                    Neighbor::new(distance + 1, new_keys_needed.clone()),
                );
                continue;
            }

            if let Some(door) = doors.get(&new_pos) {
                new_keys_needed.insert(*door);
            }

            queue.push_back((new_x, new_y, distance + 1, new_keys_needed.clone()));
        }
    }

    return neighbors;
}

#[derive(Debug, Clone)]
struct Neighbor {
    distance: i64,
    required: u32,
}

impl Neighbor {
    fn new(distance: i64, required: HashSet<char>) -> Neighbor {
        let mut req = 0;
        for r in required {
            req |= 1 << (r as u32 - 'A' as u32);
        }
        Neighbor {
            distance: distance,
            required: req,
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "18".to_string();

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
        "\nPart 1:\nShortest path to collect all keys: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nShortest path to collect all keys: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}