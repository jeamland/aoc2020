use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 7")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let mut contained_by: HashMap<String, HashSet<String>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let subject = line.split(" bags contain").nth(0).unwrap().to_string();

        let contents = line.split("contain ").nth(1).unwrap();
        if contents == "no other bags." {
            continue;
        }

        for consist in contents.strip_suffix('.').unwrap().split(", ") {
            let consist = consist.rsplitn(2, ' ').nth(1).unwrap();
            let consist: Vec<&str> = consist.splitn(2, ' ').collect();
            let containee = consist[1].to_string();

            if let Some(set) = contained_by.get_mut(&containee) {
                set.insert(subject.clone());
            } else {
                let mut set = HashSet::new();
                set.insert(subject.clone());
                contained_by.insert(containee, set);
            }
        }
    }

    let mut iter_stack = Vec::new();
    let mut container_set = HashSet::new();
    iter_stack.push(contained_by.get("shiny gold").unwrap().iter());

    while !iter_stack.is_empty() {
        match iter_stack[0].next() {
            Some(c) => {
                if let Some(l) = contained_by.get(c) {
                    iter_stack.push(l.iter());
                }
                container_set.insert(c);
            }
            None => {
                iter_stack.remove(0);
                continue;
            }
        };
    }

    println!("{} candidates", container_set.len());

    Ok(())
}
