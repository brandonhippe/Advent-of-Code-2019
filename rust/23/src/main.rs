use intcode_rust::Intcode;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let program = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());

    let (tx, rx): (Sender<(i64, i64, i64)>, Receiver<(i64, i64, i64)>) = channel();
    let mut packet_tx: Vec<Sender<(i64, i64)>> = Vec::new();

    for i in 0..50 {
        let (p_tx, p_rx): (Sender<(i64, i64)>, Receiver<(i64, i64)>) = channel();
        packet_tx.push(p_tx.clone());

        let mut nic = program.clone();
        let tx = tx.clone();

        thread::spawn(move || {
            nic.set_input(i as i64);
            nic.run();

            let mut finished = false;
            while !finished {
                thread::sleep(std::time::Duration::from_nanos(1));

                if let Ok((x, y)) = p_rx.try_recv() {
                    nic.set_input(x);
                    nic.run();
                    nic.set_input(y);
                } else {
                    nic.set_input(-1);

                }
                nic.run();

                let output = nic.get_output();

                for ix in (0..output.len()).step_by(3) {
                    let addr = output[ix];
                    let x = output[ix + 1];
                    let y = output[ix + 2];

                    if let Err(_) = tx.send((addr, x, y)) {
                        finished = true;
                        break;
                    }
                }

                nic.clear_output();
            }
        });
    }

    let mut result = 0;
    while result == 0 {
        for (addr, x, y) in &rx {
            if addr == 255 {
                result = y;
                break;
            }

            packet_tx[addr as usize].send((x, y)).unwrap();
        }
    }

    return result;
}

fn part2(contents: String) -> i64 {
    let network_size: i64 = 50;
    let program = Intcode::new(contents.split(",").map(|x| x.parse().unwrap()).collect());

    let (tx, rx): (Sender<(i64, i64, i64)>, Receiver<(i64, i64, i64)>) = channel();
    let (idle_tx, idle_rx): (Sender<(i64, i64)>, Receiver<(i64, i64)>) = channel();
    let mut packet_tx: Vec<Sender<(i64, i64)>> = Vec::new();
    let mut idle: Vec<i64> = vec![0; network_size as usize];

    for i in 0..network_size {
        let (p_tx, p_rx): (Sender<(i64, i64)>, Receiver<(i64, i64)>) = channel();
        packet_tx.push(p_tx.clone());

        let mut nic = program.clone();
        let tx = tx.clone();
        let idle_tx = idle_tx.clone();

        thread::spawn(move || {
            nic.set_input(i as i64);
            nic.run();

            let mut idle = false;
            let mut finished = false;

            while !finished {
                thread::sleep(std::time::Duration::from_nanos(1));

                if let Ok((x, y)) = p_rx.try_recv() {
                    if idle {
                        if let Err(_) = idle_tx.send((i, 0)) {
                            break;
                        }
                    }

                    idle = false;
                    nic.set_input(x);
                    nic.run();
                    nic.set_input(y);
                } else {
                    if !idle {
                        if let Err(_) = idle_tx.send((i, 1)) {
                            break;
                        }
                    }

                    idle = true;
                    nic.set_input(-1);
                }
                nic.run();

                let output = nic.get_output();

                for ix in (0..output.len()).step_by(3) {
                    let addr = output[ix];
                    let x = output[ix + 1];
                    let y = output[ix + 2];

                    if let Err(_) = tx.send((addr, x, y)) {
                        finished = true;
                        break;
                    }
                }

                nic.clear_output();
            }
        });
    }

    let mut nat: (i64, i64) = (0, 0);
    let mut last_10: Vec<i64> = (0..10).collect();

    loop {
        while let Ok((addr, x, y)) = rx.try_recv() {
            if addr == 255 {
                nat = (x, y);
            } else {
                packet_tx[addr as usize].send((x, y)).unwrap();
            }
        }

        while let Ok((ix, state)) = idle_rx.try_recv() {
            idle[ix as usize] = state;
        }

        let network_idle = idle.iter().sum::<i64>() == network_size;
        if network_idle {
            if last_10.iter().all(|&x| x == nat.1) {
                break;
            }

            last_10.remove(0);
            last_10.push(nat.1);
            packet_tx[0].send(nat).unwrap();
            thread::sleep(std::time::Duration::from_nanos(1));
        }
    }

    return nat.1;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2019".to_string();
    let day = "23".to_string();

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
        "\nPart 1:\nY value of first packet sent to address 255: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nFirst Y value delivered to address 0 twice in a row: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}