use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

const MAX_ROW: u32 = 127;
const MAX_COLUMN: u32 = 7;

struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    fn seat_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

impl From<String> for Seat {
    fn from(string: String) -> Self {
        let mut low = 0;
        let mut high = MAX_ROW;

        for character in string.chars().take(7) {
            let diff = (high + 1 - low) / 2;
            if character == 'F' {
                high -= diff;
            } else {
                low += diff;
            }
        }

        let row = low;

        let mut low = 0;
        let mut high = MAX_COLUMN;

        for character in string.chars().skip(7) {
            let diff = (high + 1 - low) / 2;
            if character == 'L' {
                high -= diff;
            } else {
                low += diff;
            }
        }

        Self { row, column: low }
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
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut max_id: u32 = 0;
    let mut seats: HashSet<u32> = (0..(MAX_ROW + 1) * (MAX_COLUMN + 1)).collect();

    for line in reader.lines() {
        let line = line.unwrap();
        print!("{}", line);
        let seat = Seat::from(line);
        println!(
            " -> row {}, column {}, seat ID {}",
            seat.row,
            seat.column,
            seat.seat_id()
        );

        if seat.seat_id() > max_id {
            max_id = seat.seat_id();
        }

        seats.remove(&seat.seat_id());
    }

    println!("Max seat ID: {}", max_id);

    for seat_id in seats.iter() {
        if seats.contains(&(seat_id - 1)) {
            continue;
        }
        if seats.contains(&(seat_id + 1)) {
            continue;
        }
        println!("Seat ID is {}", seat_id);
        break;
    }

    Ok(())
}
