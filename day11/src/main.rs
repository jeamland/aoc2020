use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

#[derive(Clone, Copy, Debug, PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

impl std::fmt::Display for SeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let character = match self {
            Self::Floor => '.',
            Self::Empty => 'L',
            Self::Occupied => '#',
        };

        write!(f, "{}", character)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct SeatingArea {
    floor: Vec<Vec<SeatState>>,
}

impl Eq for SeatingArea {}

impl<I: Iterator<Item = std::io::Result<String>>> From<I> for SeatingArea {
    fn from(lines: I) -> Self {
        let mut floor = Vec::new();

        for line in lines {
            let line = line.unwrap();
            let mut row = Vec::new();

            for character in line.chars() {
                let position = match character {
                    '.' => SeatState::Floor,
                    'L' => SeatState::Empty,
                    _ => panic!("bad input"),
                };
                row.push(position);
            }

            floor.push(row);
        }

        Self { floor }
    }
}

impl std::fmt::Display for SeatingArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in self.floor.iter() {
            for state in row {
                write!(f, "{}", state)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl SeatingArea {
    fn occupied_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let max_y = self.floor.len() - 1;
        let max_x = self.floor[0].len() - 1;

        let xrange = match x {
            0 => (0..2),
            v if v == max_x => (max_x - 1..max_x + 1),
            v => (v - 1..v + 2),
        };

        let yrange = match y {
            0 => (0..2),
            v if v == max_y => (max_y - 1..max_y + 1),
            v => (v - 1..v + 2),
        };

        for cy in yrange {
            for cx in xrange.clone() {
                if (cx, cy) == (x, y) {
                    continue;
                }

                if self.floor[cy][cx] == SeatState::Occupied {
                    count += 1;
                }
            }
        }

        count
    }

    fn visibly_occupied_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dx in -1..2 {
            for dy in -1..2 {
                if (dx, dy) == (0, 0) {
                    continue;
                }

                let mut cx = (x as i32) + dx;
                let mut cy = (y as i32) + dy;

                while cx >= 0
                    && cx < (self.floor[0].len() as i32)
                    && cy >= 0
                    && cy < (self.floor.len() as i32)
                {
                    match self.floor[cy as usize][cx as usize] {
                        SeatState::Floor => (),
                        SeatState::Empty => break,
                        SeatState::Occupied => {
                            count += 1;
                            break;
                        }
                    };

                    cx += dx;
                    cy += dy;
                }
            }
        }

        count
    }

    fn run_cycle_v1(&self) -> Self {
        let mut floor = Vec::new();

        for (y, row) in self.floor.iter().enumerate() {
            let mut new_row = Vec::new();

            for (x, state) in row.iter().copied().enumerate() {
                let new_state = match state {
                    SeatState::Floor => SeatState::Floor,
                    SeatState::Empty => {
                        if self.occupied_neighbours(x, y) == 0 {
                            SeatState::Occupied
                        } else {
                            SeatState::Empty
                        }
                    }
                    SeatState::Occupied => {
                        if self.occupied_neighbours(x, y) >= 4 {
                            SeatState::Empty
                        } else {
                            SeatState::Occupied
                        }
                    }
                };

                new_row.push(new_state);
            }

            floor.push(new_row);
        }

        Self { floor }
    }

    fn run_cycle_v2(&self) -> Self {
        let mut floor = Vec::new();

        for (y, row) in self.floor.iter().enumerate() {
            let mut new_row = Vec::new();

            for (x, state) in row.iter().copied().enumerate() {
                let new_state = match state {
                    SeatState::Floor => SeatState::Floor,
                    SeatState::Empty => {
                        if self.visibly_occupied_neighbours(x, y) == 0 {
                            SeatState::Occupied
                        } else {
                            SeatState::Empty
                        }
                    }
                    SeatState::Occupied => {
                        if self.visibly_occupied_neighbours(x, y) >= 5 {
                            SeatState::Empty
                        } else {
                            SeatState::Occupied
                        }
                    }
                };

                new_row.push(new_state);
            }

            floor.push(new_row);
        }

        Self { floor }
    }

    fn occupied(&self) -> usize {
        let mut count = 0;

        for row in self.floor.iter() {
            for state in row {
                if *state == SeatState::Occupied {
                    count += 1;
                }
            }
        }

        count
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

    let floor = SeatingArea::from(reader.lines());
    println!("{}", floor);

    let mut f1 = floor.clone();
    let mut f2 = floor.run_cycle_v1();

    while f1 != f2 {
        f1 = f2;
        f2 = f1.run_cycle_v1();
    }
    println!("{}", f2);
    println!("{} occupied", f2.occupied());
    println!();

    let mut f1 = floor.clone();
    let mut f2 = floor.run_cycle_v2();

    while f1 != f2 {
        f1 = f2;
        f2 = f1.run_cycle_v2();
    }
    println!("{}", f2);
    println!("{} occupied", f2.occupied());

    Ok(())
}
