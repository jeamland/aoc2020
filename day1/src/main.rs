use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 1")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut values: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let value = u32::from_str(&line).unwrap();
        values.push(value);
    }

    for (i, x) in values.iter().enumerate() {
        for y in values.iter().skip(i + 1) {
            println!("{} + {} = {}", x, y, x + y);
            if x + y == 2020 {
                println!("{} + {} = {} -- {} * {} = {}", x, y, x + y, x, y, x * y);
                return Ok(());
            }
        }
    }

    Ok(())
}
