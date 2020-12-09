use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};
use itertools::Itertools;

struct NumberBuffer {
    buffer: Vec<usize>,
    preamble: usize,
}

impl NumberBuffer {
    fn new(preamble: usize) -> Self {
        Self {
            buffer: Vec::new(),
            preamble,
        }
    }

    fn push(&mut self, number: usize) {
        self.buffer.push(number);
        if self.buffer.len() > self.preamble {
            self.buffer.remove(0);
        }
    }

    fn ready(&self) -> bool {
        self.buffer.len() >= self.preamble
    }

    fn is_sum_of_preamble_pair(&self, number: usize) -> bool {
        for pair in self.buffer.iter().combinations(2) {
            if number == pair.iter().copied().sum() {
                return true;
            }
        }

        false
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 4")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("PREAMBLE")
                .short("p")
                .long("preamble")
                .help("Preamble length")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);
    let numbers: Vec<usize> = reader
        .lines()
        .map(|l| usize::from_str(l.unwrap().as_str()).unwrap())
        .collect();

    let preamble = usize::from_str(matches.value_of("PREAMBLE").unwrap()).unwrap();
    let mut buffer = NumberBuffer::new(preamble);
    let mut magic_number = 0;

    for number in numbers.iter().copied() {
        if buffer.ready() && !buffer.is_sum_of_preamble_pair(number) {
            println!("{}", number);
            magic_number = number;
            break;
        }

        buffer.push(number);
    }

    let mut run: Vec<usize> = Vec::new();

    for number in numbers.iter().copied() {
        let mut sum: usize = run.iter().copied().sum();
        while sum > magic_number {
            run.remove(0);
            sum = run.iter().copied().sum();
        }

        if sum == magic_number {
            break;
        }

        run.push(number);
    }

    println!(
        "{:?} {}",
        run,
        run.iter().min().unwrap() + run.iter().max().unwrap()
    );

    Ok(())
}
