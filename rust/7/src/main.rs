use intcode_rust::Intcode;
use itertools::Itertools;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut max_sum: i64 = 0;
    for line in contents.lines() {
        let amp = Intcode::new(
            line.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );

        let mut max_output = 0;

        for input in (0..5).permutations(5) {
            let mut last_output = 0;
            for i in 0..5 {
                let mut inputs: Vec<i64> = vec![last_output, input[i]];
                let mut temp_amp = amp.clone();

                while !temp_amp.run() {
                    if inputs.len() == 0 {
                        if temp_amp.get_output().len() != 0 {
                            break;
                        }

                        panic!("Program did not halt");
                    }

                    temp_amp.set_input(inputs.pop().unwrap());
                }


                last_output = temp_amp.get_output()[0];
            }

            if last_output > max_output {
                max_output = last_output;
            }
        }

        max_sum += max_output;
    }

    return max_sum;
}

fn part2(contents: String) -> i64 {
    let mut max_sum: i64 = 0;
    for line in contents.lines() {
        let amp = Intcode::new(
            line.split(",")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );

        let mut max_output = 0;

        for input in (5..10).permutations(5) {
            let mut amps: Vec<Intcode> = Vec::new();
            for i in 0..5 {
                let mut temp_amp = amp.clone();
                temp_amp.set_input(input[i]);
                temp_amp.run();
                amps.push(temp_amp);
            }

            let mut outputs: Vec<Vec<i64>> = vec![vec![]; 5];
            outputs[0].push(0);

            let mut halted = false;

            while !halted {
                halted = true;
                for i in 0..5 {
                    let mut temp_amp = amps[i].clone();
                    temp_amp.set_input(outputs[i].pop().unwrap());

                    if !temp_amp.run() {
                        halted = false;
                    }

                    outputs[(i + 1) % 5].push(temp_amp.get_output().pop().unwrap());
                    amps[i] = temp_amp;
                }
            }

            if outputs[0][0] > max_output {
                max_output = outputs[0][0];
            }
        }

        max_sum += max_output;
    }

    return max_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 162741);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 139647945);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "7".to_string();

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
        "\nPart 1:\nMaximum signal: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMaximum signal: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}