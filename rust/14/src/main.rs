use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut reactions: HashMap<String, (i64, Vec<(i64, String)>)> = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.split(" => ");
        let inputs = parts.next().unwrap();
        let output = parts.next().unwrap();

        let mut output_parts = output.split(' ');
        let output_amount = output_parts.next().unwrap().parse::<i64>().unwrap();
        let output_chemical = output_parts.next().unwrap();

        let mut inputs_vec: Vec<(i64, String)> = Vec::new();
        for input in inputs.split(", ") {
            let mut input_parts = input.split(' ');
            let input_amount = input_parts.next().unwrap().parse::<i64>().unwrap();
            let input_chemical = input_parts.next().unwrap();
            inputs_vec.push((input_amount, input_chemical.to_string()));
        }

        reactions.insert(output_chemical.to_string(), (output_amount, inputs_vec));
    }

    return ore_needed(&reactions, 1).0;
}

fn part2(contents: String) -> i64 {
    let mut reactions: HashMap<String, (i64, Vec<(i64, String)>)> = HashMap::new();


    for line in contents.lines() {
        let mut parts = line.split(" => ");
        let inputs = parts.next().unwrap();
        let output = parts.next().unwrap();

        let mut output_parts = output.split(' ');
        let output_amount = output_parts.next().unwrap().parse::<i64>().unwrap();
        let output_chemical = output_parts.next().unwrap();

        let mut inputs_vec: Vec<(i64, String)> = Vec::new();
        for input in inputs.split(", ") {
            let mut input_parts = input.split(' ');
            let input_amount = input_parts.next().unwrap().parse::<i64>().unwrap();
            let input_chemical = input_parts.next().unwrap();
            inputs_vec.push((input_amount, input_chemical.to_string()));
        }

        reactions.insert(output_chemical.to_string(), (output_amount, inputs_vec));
    }

    let mut total_fuel = 0;
    let mut ore_remaining: i64 = 1000000000000;

    let one_fuel = ore_needed(&reactions, 1).0;

    loop {
        let mut fuel_to_make = ore_remaining / one_fuel;
        let mut extras: HashMap<String, i64>;
        let mut ore_used: i64;
        let continue_loop: bool;

        loop {
            let tup = ore_needed(&reactions, fuel_to_make);
            ore_used = tup.0;
            extras = tup.1;

            if ore_used >= ore_remaining || extras.values().sum::<i64>() == 0 {
                continue_loop = extras.values().sum::<i64>() == 0;
                break;
            }

            fuel_to_make += (ore_remaining - ore_used) / one_fuel + 1;
        }

        if !continue_loop {
            total_fuel += fuel_to_make - 1;
            break;
        }

        let amt = ore_remaining / ore_used;
        total_fuel += fuel_to_make * amt;
        ore_remaining -= ore_used * amt;
    }

    return total_fuel;
}

fn ore_needed(
    reactions: &HashMap<String, (i64, Vec<(i64, String)>)>,
    amt: i64,
) -> (i64, HashMap<String, i64>) {
    let mut ore_needed = 0;
    let mut needed: VecDeque<(i64, String)> = vec![(amt, "FUEL".to_string())].into();
    let mut extras: HashMap<String, i64> = HashMap::new();

    while !needed.is_empty() {
        let (needed_amount, needed_chemical) = needed.pop_front().unwrap();

        if needed_chemical == "ORE" {
            ore_needed += needed_amount;
            continue;
        }

        let mut needed_amount = needed_amount;
        if let Some(extra_amount) = extras.get_mut(&needed_chemical) {
            if *extra_amount >= needed_amount {
                *extra_amount -= needed_amount;
                continue;
            } else {
                needed_amount -= *extra_amount;
                *extra_amount = 0;
            }
        }

        let (output_amount, inputs) = reactions.get(&needed_chemical).unwrap();
        let mult = (needed_amount as f64 / *output_amount as f64).ceil() as i64;
        let extra = (mult * output_amount) - needed_amount;
        *extras.entry(needed_chemical.clone()).or_insert(0) += extra;

        for (input_amount, input_chemical) in inputs {
            needed.push_back((input_amount * mult, input_chemical.clone()));
        }
    }

    return (ore_needed, extras);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let mut contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 165);

        contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 13312);

        contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 180697);

        contents =
            fs::read_to_string("example3.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 2210736);
    }

    #[test]
    fn p2_test() {
        let mut contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 82892753);

        contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 5586022);

        contents =
            fs::read_to_string("example3.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 460664);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "14".to_string();

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
        "\nPart 1:\nOre needed for 1 fuel: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nFuel made with 1 trillion ore: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}