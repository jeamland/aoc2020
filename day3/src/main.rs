use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

const SLOPES: &[(usize, usize)] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

struct TreeField(Vec<Vec<bool>>);

impl TreeField {
    fn new(data: Vec<Vec<bool>>) -> Self {
        Self(data)
    }

    fn count_trees(&self, right: usize, down: usize) -> usize {
        let row_len = self.0[0].len();

        let mut row_pos = 0;
        let mut tree_count = 0;

        for (i, row) in self.0.iter().enumerate() {
            if i % down != 0 {
                continue;
            }

            if row[row_pos] {
                tree_count += 1;
            }

            row_pos = (row_pos + right) % row_len;
        }

        tree_count
    }
}

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

    for line in reader.lines() {
        let line = line.unwrap();

        let row: Vec<bool> = line.chars().map(|c| c == '#').collect();
        field.push(row);
    }

    let field = TreeField::new(field);
    let mut product = 1;

    for (right, down) in SLOPES.iter() {
        let tree_count = field.count_trees(*right, *down);
        println!("Right {}, down {}: {} trees", right, down, tree_count);
        product *= tree_count;
    }
    println!("Final product: {}", product);

    Ok(())
}
