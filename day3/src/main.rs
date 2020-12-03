use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 3")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut field: Vec<Vec<bool>> = Vec::new();
    let mut row_len = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let row: Vec<bool> = line.chars().map(|c| c == '#').collect();
        row_len = row.len();
        field.push(row);
    }

    let mut row_pos = 0;
    let mut tree_count = 0;
    for row in field.iter() {
        if row[row_pos] {
            tree_count += 1;
        }

        row_pos = (row_pos + 3) % row_len;
    }

    println!("{} trees", tree_count);

    Ok(())
}
