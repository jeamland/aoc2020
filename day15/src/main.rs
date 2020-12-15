use std::collections::HashMap;
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

    let mut n1 = numbers.clone();
    for _ in n1.len()..2020 {
        let number = *n1.last().unwrap();
        match n1.iter().rposition(|v| *v == number) {
            None => n1.push(0),
            Some(n) => {
                let value = match n1[..n].iter().rposition(|v| *v == number) {
                    None => 0,
                    Some(m) => n - m,
                };
                n1.push(value);
            }
        }
    }

    println!("{}", n1.last().unwrap());

    let mut cache = HashMap::new();

    let mut number = numbers.pop().unwrap();
    for (c, n) in numbers.iter().enumerate() {
        cache.insert(*n, c);
    }

    for c in numbers.len()..30000000 - 1 {
        if c % 1000000 == 0 {
            println!("... {:8}", c);
        }
        let old_number = number;

        number = match cache.get(&number) {
            None => 0,
            Some(v) => c - *v,
        };
        cache.insert(old_number, c);
    }

    println!("{}", number);
}
