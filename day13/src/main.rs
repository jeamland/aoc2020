use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

fn bezout_coefficient(a: usize, b: usize) -> (i128, i128) {
    let mut old_remainder = a as i128;
    let mut remainder = b as i128;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while remainder != 0 {
        let quotient = old_remainder / remainder;
        let (new_old_r, new_r) = (remainder, old_remainder - quotient * remainder);
        remainder = new_r;
        old_remainder = new_old_r;
        let (new_old_s, new_s) = (s, old_s - quotient * s);
        s = new_s;
        old_s = new_old_s;
        let (new_old_t, new_t) = (t, old_t - quotient * t);
        t = new_t;
        old_t = new_old_t;
    }

    (old_s, old_t)
}

fn existence(rem_a: usize, mod_a: usize, rem_b: usize, mod_b: usize) -> usize {
    let (ac, bc) = bezout_coefficient(mod_a, mod_b);
    let ra = ac * (mod_a * rem_b) as i128;
    let rb = bc * (mod_b * rem_a) as i128;
    let m = (mod_a * mod_b) as i128;
    let mut result = ra + rb;

    while result < 0 {
        result += m;
    }
    result %= result;
    result as usize
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
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let earliest = usize::from_str(&lines[0]).unwrap();
    let busses: Vec<Option<usize>> = lines[1]
        .split(',')
        .map(|v| match v {
            x if x == "x" => None,
            x => Some(usize::from_str(x).unwrap()),
        })
        .collect();

    let mut wait_times: Vec<(usize, usize)> = busses
        .iter()
        .filter(|v| v.is_some())
        .copied()
        .map(|b| {
            let b = b.unwrap();
            (b, b - (earliest % b))
        })
        .collect();
    wait_times.sort_by_key(|(_, w)| *w);
    let (bus_id, wait_time) = wait_times[0];
    println!("{} * {} = {}", bus_id, wait_time, bus_id * wait_time);

    println!();

    let mut values: Vec<(usize, usize)> = busses
        .iter()
        .enumerate()
        .filter(|(_, b)| b.is_some())
        .map(|(i, b)| (i, b.unwrap()))
        .collect();

    while values.len() > 1 {
        let (rem_a, mod_a) = values.remove(0);
        let (rem_b, mod_b) = values.remove(0);

        values.push((existence(rem_a, mod_a, rem_b, mod_b), mod_a * mod_b));
    }

    let (r, m) = values[0];
    println!("t = {}", m - r);

    Ok(())
}
