use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

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
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let earliest = usize::from_str(&lines[0]).unwrap();
    let busses: Vec<usize> = lines[1]
        .split(',')
        .filter(|c| *c != "x")
        .map(|v| usize::from_str(v).unwrap())
        .collect();

    let mut wait_times: Vec<(usize, usize)> = busses
        .iter()
        .copied()
        .map(|b| (b, b - (earliest % b)))
        .collect();
    wait_times.sort_by_key(|(_, w)| *w);

    let (bus_id, wait_time) = wait_times[0];

    println!("{} * {} = {}", bus_id, wait_time, bus_id * wait_time);

    Ok(())
}
