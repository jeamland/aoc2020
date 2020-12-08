use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

#[derive(Debug)]
enum Instruction {
    Accumulate(i32),
    Jump(i32),
    Noop,
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();
        match parts[0] {
            "acc" => Self::Accumulate(i32::from_str(parts[1]).unwrap()),
            "jmp" => Self::Jump(i32::from_str(parts[1]).unwrap()),
            "nop" => Self::Noop,
            _ => Self::Noop,
        }
    }
}

struct Executor {
    program: Vec<Instruction>,
    pc: usize,
    accumulator: i32,
}

impl Executor {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            accumulator: 0,
        }
    }

    fn run(&mut self) -> i32 {
        let mut visited = HashSet::new();

        loop {
            if visited.contains(&self.pc) {
                return self.accumulator;
            } else {
                visited.insert(self.pc);
            }

            match self.program.get(self.pc).unwrap() {
                Instruction::Accumulate(value) => {
                    self.accumulator += value;
                    self.pc += 1;
                }
                Instruction::Jump(value) => {
                    self.pc = ((self.pc as i32) + value) as usize;
                }
                Instruction::Noop => {
                    self.pc += 1;
                }
            };
        }
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 8")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut program: Vec<Instruction> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        program.push(Instruction::from(line));
    }

    let mut executor = Executor::new(program);
    println!("acc = {}", executor.run());

    Ok(())
}
