use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return contents
        .chars()
        .collect::<Vec<char>>()
        .chunks(25 * 6)
        .enumerate()
        .min_by_key(|(_, layer)| layer.iter().filter(|&&c| c == '0').count())
        .map(|(i, layer)| {
            layer.iter().filter(|&&c| c == '1').count()
                * layer.iter().filter(|&&c| c == '2').count()
        })
        .unwrap() as i64;
}

fn part2(contents: String) -> String {
    let mut pixels: HashMap<(i64, i64), bool> = HashMap::new();
    for layer in contents.chars().collect::<Vec<char>>().chunks(25 * 6) {
        for (i, pixel) in layer.iter().enumerate() {
            let x = (i % 25) as i64;
            let y = (i / 25) as i64;
            if !pixels.contains_key(&(x, y)) && *pixel != '2' {
                pixels.insert((x, y), *pixel == '1');
            }
        }
    }

    let mut result = String::new();
    for y in 0..6 {
        result.push('\n');

        for x in 0..25 {
            result.push(if *pixels.get(&(x, y)).unwrap() {
                '█'
            } else {
                ' '
            });
        }
    }

    return result;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "8".to_string();

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
        "\nPart 1:\n1's * 2's on layer with fewest 0's: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMessage: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}