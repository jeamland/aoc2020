use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

#[derive(Clone, Debug)]
enum Instruction {
    Accumulate(i32),
    Jump(i32),
    Noop(i32),
}

impl Instruction {
    fn is_jump_or_noop(&self) -> bool {
        match self {
            Instruction::Accumulate(_) => false,
            Instruction::Jump(_) => true,
            Instruction::Noop(_) => true,
        }
    }
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let value = i32::from_str(parts[1]).unwrap();
        match parts[0] {
            "acc" => Self::Accumulate(value),
            "jmp" => Self::Jump(value),
            "nop" => Self::Noop(value),
            _ => Self::Noop(0),
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

    fn execute_one(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Accumulate(value) => {
                self.accumulator += value;
                self.pc += 1;
            }
            Instruction::Jump(value) => {
                self.pc = ((self.pc as i32) + value) as usize;
            }
            Instruction::Noop(_) => {
                self.pc += 1;
            }
        };
    }

    fn run(&mut self) -> i32 {
        self.pc = 0;
        self.accumulator = 0;

        let mut visited = HashSet::new();

        loop {
            if visited.contains(&self.pc) {
                return self.accumulator;
            } else {
                visited.insert(self.pc);
            }

            let instruction = self.program.get(self.pc).unwrap().clone();
            self.execute_one(&instruction);
        }
    }

    fn tweakpoints(&self) -> Vec<usize> {
        self.program
            .iter()
            .enumerate()
            .filter(|(_, insn)| insn.is_jump_or_noop())
            .map(|(pc, _)| pc)
            .collect()
    }

    fn run_tweaked(&mut self, tweakpoint: usize) -> Option<i32> {
        self.pc = 0;
        self.accumulator = 0;

        let mut visited = HashSet::new();

        loop {
            if visited.contains(&self.pc) {
                return None;
            } else {
                visited.insert(self.pc);
            }

            let mut instruction = self.program.get(self.pc).unwrap().clone();
            if self.pc == tweakpoint {
                instruction = match instruction {
                    Instruction::Jump(value) => Instruction::Noop(value),
                    Instruction::Noop(value) => Instruction::Jump(value),
                    i => i,
                };
            }

            self.execute_one(&instruction);

            match self.pc.cmp(&self.program.len()) {
                Ordering::Equal => return Some(self.accumulator),
                Ordering::Greater => return None,
                Ordering::Less => (),
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

    let mut executor = Executor::new(program.clone());
    println!("acc = {}", executor.run());

    let mut executor = Executor::new(program);
    for tweak in executor.tweakpoints() {
        if let Some(acc) = executor.run_tweaked(tweak) {
            println!("tweakpoint = {}, acc = {}", tweak, acc);
            break;
        }
    }

    Ok(())
}
