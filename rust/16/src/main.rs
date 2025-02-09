use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let base_pattern: Vec<i64> = vec![0, 1, 0, -1];
    let mut arr: Vec<i64> = contents
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    for _ in 0..100 {
        let mut new_arr = Vec::new();
        for i in 0..arr.len() {
            let mut sum = 0;

            for (j, x) in arr.iter().enumerate() {
                sum += x * base_pattern[((j + 1) / (i + 1)) % 4]
            }

            new_arr.push(sum.abs() % 10);
        }

        arr = new_arr;
    }

    return arr.iter().take(8).fold(0, |acc, x| acc * 10 + x);
}

fn part2(contents: String) -> i64 {
    let base_pattern: Vec<i64> = vec![0, 1, 0, -1];
    let mut arr: Vec<i64> = contents
        .repeat(10000)
        .chars()

        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let offset: i64 = arr.iter().take(7).fold(0, |acc, x| acc * 10 + x);
    arr = (offset as usize..arr.len()).map(|i| arr[i]).collect();

    for _ in 0..100 {
        arr = fft(arr);
    }

    return arr.iter().take(8).fold(0, |acc, x| acc * 10 + x);
}

fn fft(arr: Vec<i64>) -> Vec<i64> {
    let mut new_arr: Vec<i64> = vec![0];
    for n in arr.iter().rev() {
        new_arr.push((n + new_arr[new_arr.len() - 1]).abs() % 10);
    }

    return new_arr.iter().skip(1).rev().map(|x| *x).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        assert_eq!(
            part1("80871224585914546619083218645595".to_string()),
            24176176
        );
        assert_eq!(
            part1("19617804207202209144916044189917".to_string()),
            73745418
        );
        assert_eq!(
            part1("69317163492948606335995924319873".to_string()),
            52432133
        );
    }

    #[test]
    fn p2_test() {
        assert_eq!(
            part2("03036732577212944063491565474664".to_string()),
            84462026
        );
        assert_eq!(
            part2("02935109699940807407585447034323".to_string()),
            78725270
        );
        assert_eq!(
            part2("03081770884921959731165446850517".to_string()),
            53553731
        );
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "16".to_string();

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
        "\nPart 1:\nFirst 8 digits after 100 steps: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nFirst 8 digits of message: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}