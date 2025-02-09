use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut a: i64 = 1;
    let mut b: i64 = 0;

    let deck_size = 10007;

    for line in contents.lines() {
        let (c, d): (i64, i64) = if line.contains("stack") {
            (-1, -1)
        } else if line.contains("increment") {
            (line.split(" ").last().unwrap().parse::<i64>().unwrap(), 0)
        } else {
            (1, -line.split(" ").last().unwrap().parse::<i64>().unwrap())
        };

        a = (a * c) % deck_size;
        b = (b * c + d) % deck_size;
    }

    return (2019 * a + b) % deck_size;
}

fn part2(contents: String) -> i128 {
    let mut a: i128 = 1;
    let mut b: i128 = 0;

    let deck_size: i128 = 119315717514047;

    for line in contents.lines() {
        let (c, d): (i128, i128) = if line.contains("stack") {

            (-1, -1)
        } else if line.contains("increment") {
            (line.split(" ").last().unwrap().parse::<i128>().unwrap(), 0)
        } else {
            (1, -line.split(" ").last().unwrap().parse::<i128>().unwrap())
        };

        a = (a * c) % deck_size;
        b = (b * c + d) % deck_size;
    }

    let a_prime = mod_exp(a, 101741582076661, deck_size);
    let mut b_prime: i128 = (b * (1 - a_prime)) % deck_size;
    b_prime *= mod_inverse(1 - a, deck_size);
    b_prime %= deck_size;

    let mut result = (2020 - b_prime) % deck_size;
    result *= mod_inverse(a_prime, deck_size);
    result %= deck_size;
    result += deck_size;
    result %= deck_size;

    return result;
}

fn mod_exp(mut base: i128, mut exp: i128, m: i128) -> i128 {
    let mut result = 1;
    base %= m;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % m;
        }

        exp >>= 1;
        base = (base * base) % m;
    }

    return result;
}

fn mod_inverse(n: i128, m: i128) -> i128 {
    return mod_exp(n, m - 2, m);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "22".to_string();

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
        "\nPart 1:\nPosition of card 2019: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nCard in position 2020: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}