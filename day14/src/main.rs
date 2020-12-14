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
struct MachineV1 {
    memory: HashMap<u64, u64>,
    mask: Masker,
}

impl MachineV1 {
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

#[derive(Debug)]
struct MaskerV2 {
    mask: u64,
    floaters: Vec<usize>,
}

impl From<&str> for MaskerV2 {
    fn from(string: &str) -> Self {
        let mut mask = 0;
        let mut floaters = Vec::new();

        for (i, c) in string.chars().enumerate() {
            match c {
                'X' => {
                    floaters.push(35 - i);
                }
                '1' => {
                    mask |= 1 << (35 - i);
                }
                '0' => (),
                _ => panic!("unknown character in mask"),
            };
        }

        Self { mask, floaters }
    }
}

struct AddressGenerator {
    floaters: Vec<usize>,
    base: u64,
    counter: u64,
    done: bool,
}

impl AddressGenerator {
    fn new(mask: &MaskerV2, address: u64) -> Self {
        let base = address | mask.mask;
        Self {
            floaters: mask.floaters.clone(),
            base,
            counter: 0,
            done: false,
        }
    }
}

impl Iterator for AddressGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let mut bits = Vec::new();
        let mut address = self.base;

        self.done = true;
        for idx in 0..self.floaters.len() {
            let bit = (self.counter >> idx) & 0x1;
            if bit == 0x0 {
                self.done = false;
            }
            bits.push(bit);
        }

        for (bit, pos) in bits.iter().zip(self.floaters.iter()) {
            address = (address & !(1 << pos)) | (bit << pos);
        }

        self.counter += 1;

        Some(address)
    }
}

impl MaskerV2 {
    fn apply(&self, address: u64) -> AddressGenerator {
        AddressGenerator::new(&self, address)
    }
}

#[derive(Debug)]
struct MachineV2 {
    memory: HashMap<u64, u64>,
    mask: MaskerV2,
}

impl MachineV2 {
    fn new() -> Self {
        Self {
            memory: HashMap::new(),
            mask: MaskerV2 {
                mask: 0,
                floaters: Vec::new(),
            },
        }
    }

    fn set_mask(&mut self, masker: MaskerV2) {
        self.mask = masker;
    }

    fn set_memory(&mut self, address: u64, value: u64) {
        for address in self.mask.apply(address) {
            self.memory.insert(address, value);
        }
    }

    fn sum_values(&self) -> u64 {
        self.memory.values().copied().sum()
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 14")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut machinev1 = MachineV1::new();
    let mut machinev2 = MachineV2::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("mask =") {
            machinev1.set_mask(Masker::from(&line[7..]));
            machinev2.set_mask(MaskerV2::from(&line[7..]));
        } else if line.starts_with("mem") {
            let close = line.find(']').unwrap();
            let space = line.rfind(' ').unwrap();
            let address = u64::from_str(&line[4..close]).unwrap();
            let value = u64::from_str(&line[space + 1..]).unwrap();
            machinev1.set_memory(address, value);
            machinev2.set_memory(address, value);
        }
    }

    println!("Part 1: {}", machinev1.sum_values());
    println!("Part 2: {}", machinev2.sum_values());

    Ok(())
}
