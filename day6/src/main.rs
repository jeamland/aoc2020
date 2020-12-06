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

    let mut groups_v1: Vec<HashSet<char>> = Vec::new();
    let mut groups_v2: Vec<HashSet<char>> = Vec::new();
    let mut group: HashSet<char> = HashSet::new();
    let mut distinct: Option<HashSet<char>> = None;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            groups_v1.push(group);
            group = HashSet::new();

            groups_v2.push(distinct.unwrap());
            distinct = None;

            continue;
        }

        group.extend(line.chars());

        let answers: HashSet<char> = line.chars().collect();
        if distinct.is_none() {
            distinct = Some(answers);
        } else {
            distinct = Some(
                distinct
                    .unwrap()
                    .intersection(&answers)
                    .map(|c| *c)
                    .collect(),
            );
        }
    }

    groups_v1.push(group);
    groups_v2.push(distinct.unwrap());

    let sum: usize = groups_v1.iter().map(|g| g.len()).sum();
    println!("Sum v1: {}", sum);

    let sum: usize = groups_v2.iter().map(|g| g.len()).sum();
    println!("Sum v2: {}", sum);

    Ok(())
}
