use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

#[derive(Debug)]
struct Masker {
    mask: u64,
    fill: u64,
}

impl From<&str> for Masker {
    fn from(string: &str) -> Self {
        let mut mask = 0;
        let mut fill = 0;

        for (i, c) in string.chars().enumerate() {
            match c {
                'X' => {
                    mask |= 1 << (35 - i);
                }
                '1' => {
                    fill |= 1 << (35 - i);
                }
                '0' => (),
                _ => panic!("unknown character in mask"),
            };
        }

        Self { mask, fill }
    }
}

impl Masker {
    fn apply(&self, value: u64) -> u64 {
        (value & self.mask) | self.fill
    }
}

#[derive(Debug)]
struct Machine {
    memory: HashMap<u64, u64>,
    mask: Masker,
}

impl Machine {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
            mask: Masker { mask: 0, fill: 0 },
        }
    }

    fn set_mask(&mut self, masker: Masker) {
        self.mask = masker;
    }

    fn set_memory(&mut self, address: u64, value: u64) {
        self.memory.insert(address, self.mask.apply(value));
    }

    fn sum_values(&self) -> u64 {
        self.memory.values().copied().sum()
    }
}

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

    let mut machine = Machine::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("mask =") {
            machine.set_mask(Masker::from(&line[7..]));
        } else if line.starts_with("mem") {
            let close = line.find(']').unwrap();
            let space = line.rfind(' ').unwrap();
            let address = u64::from_str(&line[4..close]).unwrap();
            let value = u64::from_str(&line[space + 1..]).unwrap();
            machine.set_memory(address, value);
        }
    }

    println!("{}", machine.sum_values());

    Ok(())
}
