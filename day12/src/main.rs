use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

#[derive(Clone, Copy, Debug)]
enum Action {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::North(v) => write!(f, "N{}", v),
            Self::South(v) => write!(f, "S{}", v),
            Self::East(v) => write!(f, "E{}", v),
            Self::West(v) => write!(f, "W{}", v),
            Self::Left(v) => write!(f, "L{}", v),
            Self::Right(v) => write!(f, "R{}", v),
            Self::Forward(v) => write!(f, "F{}", v),
        }
    }
}

impl From<String> for Action {
    fn from(s: String) -> Self {
        let mut si = s.chars();
        let action = si.next().unwrap();
        let value: String = si.collect();
        let value = usize::from_str(value.as_str()).unwrap();
        match action {
            'N' => Self::North(value),
            'S' => Self::South(value),
            'E' => Self::East(value),
            'W' => Self::West(value),
            'L' => Self::Left(value),
            'R' => Self::Right(value),
            'F' => Self::Forward(value),
            _ => panic!("unknown action"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl std::ops::AddAssign<usize> for Heading {
    fn add_assign(&mut self, rhs: usize) {
        match (&self, rhs) {
            (Heading::North, 90) => *self = Heading::West,
            (Heading::North, 180) => *self = Heading::South,
            (Heading::North, 270) => *self = Heading::East,
            (Heading::South, 90) => *self = Heading::East,
            (Heading::South, 180) => *self = Heading::North,
            (Heading::South, 270) => *self = Heading::West,
            (Heading::East, 90) => *self = Heading::North,
            (Heading::East, 180) => *self = Heading::West,
            (Heading::East, 270) => *self = Heading::South,
            (Heading::West, 90) => *self = Heading::South,
            (Heading::West, 180) => *self = Heading::East,
            (Heading::West, 270) => *self = Heading::North,
            _ => panic!("bad heading"),
        };
    }
}

impl std::ops::SubAssign<usize> for Heading {
    fn sub_assign(&mut self, rhs: usize) {
        match (&self, rhs) {
            (Heading::North, 90) => *self = Heading::East,
            (Heading::North, 180) => *self = Heading::South,
            (Heading::North, 270) => *self = Heading::West,
            (Heading::South, 90) => *self = Heading::West,
            (Heading::South, 180) => *self = Heading::North,
            (Heading::South, 270) => *self = Heading::East,
            (Heading::East, 90) => *self = Heading::South,
            (Heading::East, 180) => *self = Heading::West,
            (Heading::East, 270) => *self = Heading::North,
            (Heading::West, 90) => *self = Heading::North,
            (Heading::West, 180) => *self = Heading::East,
            (Heading::West, 270) => *self = Heading::South,
            _ => panic!("bad heading"),
        };
    }
}

impl fmt::Display for Heading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::North => write!(f, "N"),
            Self::South => write!(f, "S"),
            Self::East => write!(f, "E"),
            Self::West => write!(f, "W"),
        }
    }
}

struct Ferry {
    north: i32,
    east: i32,
    heading: Heading,
}

impl Ferry {
    fn new() -> Self {
        Self {
            north: 0,
            east: 0,
            heading: Heading::East,
        }
    }

    fn process_action(&mut self, action: Action) {
        match action {
            Action::North(v) => self.north += v as i32,
            Action::South(v) => self.north -= v as i32,
            Action::East(v) => self.east += v as i32,
            Action::West(v) => self.east -= v as i32,
            Action::Left(v) => self.heading += v,
            Action::Right(v) => self.heading -= v,
            Action::Forward(v) => match self.heading {
                Heading::North => self.north += v as i32,
                Heading::South => self.north -= v as i32,
                Heading::East => self.east += v as i32,
                Heading::West => self.east -= v as i32,
            },
        }

        println!(
            "{} -> ({} {} {})",
            action, self.north, self.east, self.heading
        );
    }

    fn distance(&self) -> usize {
        let distance = self.north.abs() + self.east.abs();
        distance as usize
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 12")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut ferry = Ferry::new();

    for line in reader.lines() {
        let action = Action::from(line.unwrap());
        ferry.process_action(action);
    }

    println!("distance: {}", ferry.distance());

    Ok(())
}
