use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter;
use std::str::FromStr;

use clap::{App, Arg};

fn pairwise<T>(iter: T) -> impl Iterator<Item = (usize, usize)>
where
    T: Iterator<Item = usize> + Clone,
{
    let iter2 = iter::once(0).chain(iter.clone());
    iter2.zip(iter)
}

fn run_counts(numbers: &[usize]) -> Vec<usize> {
    let mut runs = Vec::new();
    let mut count = 0;

    for difference in pairwise(numbers.iter().copied()).map(|(a, b)| b - a) {
        match difference {
            1 => count += 1,
            _ => {
                runs.push(count);
                count = 0;
            }
        }
    }

    if count != 0 {
        runs.push(count);
    }

    runs
}

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 10")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut numbers: Vec<usize> = reader
        .lines()
        .map(|l| usize::from_str(l.unwrap().as_str()).unwrap())
        .collect();

    numbers.sort_unstable();

    let mut count_1 = 0;
    let mut count_3 = 1;

    for difference in pairwise(numbers.iter().copied()).map(|(a, b)| b - a) {
        match difference {
            1 => count_1 += 1,
            3 => count_3 += 1,
            _ => (),
        };
    }

    println!("{} x 1, {} x 3 -> {}", count_1, count_3, count_1 * count_3);

    let counts: Vec<usize> = run_counts(&numbers)
        .iter()
        .copied()
        .filter(|c| *c > 1)
        .map(|c| ((c * (c - 1)) / 2) + 1)
        .collect();
    println!("{}", counts.iter().copied().product::<usize>());

    Ok(())
}
