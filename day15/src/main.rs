use std::str::FromStr;

use clap::{App, Arg};

fn main() {
    let matches = App::new("AOC2020 Day 15")
        .arg(
            Arg::with_name("start")
                .help("Starting numbers")
                .required(true)
                .index(1),
        )
        .get_matches();

    let mut numbers: Vec<usize> = matches
        .value_of("start")
        .unwrap()
        .split(',')
        .map(|v| usize::from_str(v).unwrap())
        .collect();

    for _ in numbers.len()..2020 {
        let number = *numbers.last().unwrap();
        match numbers.iter().rposition(|v| *v == number) {
            None => numbers.push(0),
            Some(n) => {
                let value = match numbers[..n].iter().rposition(|v| *v == number) {
                    None => 0,
                    Some(m) => n - m,
                };
                numbers.push(value);
            }
        }
    }

    println!("{}", numbers.last().unwrap());
}
