use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

struct EnergySource {
    volume: HashMap<(isize, isize, isize), bool>,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl EnergySource {
    fn add(&mut self, x: isize, y: isize, z: isize, active: bool) {
        self.volume.insert((x, y, z), active);
        if active {
            self.x_min = self.x_min.min(x);
            self.x_max = self.x_max.max(x);
            self.y_min = self.y_min.min(y);
            self.y_max = self.y_max.max(y);
            self.z_min = self.z_min.min(z);
            self.z_max = self.z_max.max(z);
        }
    }

    fn print(&self) {
        for z in self.z_min..self.z_max + 1 {
            println!("z={}", z);
            for y in self.y_min..self.y_max + 1 {
                for x in self.x_min..self.x_max + 1 {
                    match self.volume.get(&(x, y, z)).unwrap() {
                        true => print!("#"),
                        false => print!("."),
                    }
                }
                println!();
            }
            println!();
        }
    }

    fn is_cube_active(&self, x: isize, y: isize, z: isize) -> bool {
        match self.volume.get(&(x, y, z)) {
            None => false,
            Some(v) => *v,
        }
    }

    fn active_neighbours(&self, x: isize, y: isize, z: isize) -> usize {
        let mut count = 0;

        for cz in z - 1..z + 2 {
            for cy in y - 1..y + 2 {
                for cx in x - 1..x + 2 {
                    if (cx, cy, cz) == (x, y, z) {
                        continue;
                    }

                    if self.is_cube_active(cx, cy, cz) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn cycle(&self) -> Self {
        let mut source = Self {
            volume: HashMap::new(),
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
            z_min: 0,
            z_max: 0,
        };

        for z in self.z_min - 1..self.z_max + 2 {
            for y in self.y_min - 1..self.y_max + 2 {
                for x in self.x_min - 1..self.x_max + 2 {
                    let active = self.is_cube_active(x, y, z);
                    let active_neighbours = self.active_neighbours(x, y, z);
                    let active = match (active, active_neighbours) {
                        (true, 2) => true,
                        (true, 3) => true,
                        (true, _) => false,
                        (false, 3) => true,
                        (false, _) => false,
                    };
                    source.add(x, y, z, active);
                }
            }
        }

        source
    }

    fn active_count(&self) -> usize {
        self.volume.values().filter(|v| **v).count()
    }
}

impl<I: Iterator<Item = std::io::Result<String>>> From<I> for EnergySource {
    fn from(iterator: I) -> Self {
        let mut source = Self {
            volume: HashMap::new(),
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
            z_min: 0,
            z_max: 0,
        };

        for (y, line) in iterator.map(|l| l.unwrap()).enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let active = match ch {
                    '.' => false,
                    '#' => true,
                    _ => panic!("invalid character"),
                };

                source.add(x as isize, y as isize, 0, active);
            }
        }

        source
    }
}

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 17")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut source = EnergySource::from(reader.lines());
    source.print();

    for cycle in 1..7 {
        println!("Cycle {}:", cycle);
        source = source.cycle();
        source.print();
    }

    println!("active count: {}", source.active_count());

    Ok(())
}
