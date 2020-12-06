use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 6")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut group: HashSet<char> = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            groups.push(group);
            group = HashSet::new();
            continue;
        }

        group.extend(line.chars());
    }

    if !group.is_empty() {
        groups.push(group);
    }

    let sum: usize = groups.iter().map(|g| g.len()).sum();
    println!("Sum: {}", sum);

    Ok(())
}
