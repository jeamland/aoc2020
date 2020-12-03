use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 2")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut good_count_v1 = 0;
    let mut good_count_v2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let bits: Vec<&str> = line.split_whitespace().collect();

        let range: Vec<&str> = bits[0].split("-").collect();
        let low = usize::from_str(range[0]).unwrap();
        let high = usize::from_str(range[1]).unwrap();
        let character = bits[1].chars().nth(0).unwrap();
        let password = bits[2];

        println!("{} - {} {} {}", low, high, character, password);

        let char_count = password.chars().filter(|c| *c == character).count();
        if char_count >= low && char_count <= high {
            good_count_v1 += 1;
        }

        match (
            password.chars().nth(low - 1),
            password.chars().nth(high - 1),
        ) {
            (Some(a), Some(b)) if a == character && a != b => {
                good_count_v2 += 1;
            }
            (Some(a), Some(b)) if b == character && a != b => {
                good_count_v2 += 1;
            }
            _ => (),
        };
    }

    println!("{} good passwords (v1)", good_count_v1);
    println!("{} good passwords (v2)", good_count_v2);

    Ok(())
}
