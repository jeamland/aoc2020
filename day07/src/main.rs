use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

fn count_bags<S>(bag_list: &HashMap<String, Vec<(u32, String)>>, start: S) -> u32
where
    S: ToString,
{
    let start = start.to_string();
    let mut total = 0;

    if !bag_list.contains_key(&start) {
        return 0;
    }

    for (count, colour) in bag_list.get(&start).unwrap() {
        total += count + count * count_bags(bag_list, colour);
    }

    total
}

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
    let mut container_tree: HashMap<String, Vec<(u32, String)>> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let subject = line.split(" bags contain").next().unwrap().to_string();

        let contents = line.split("contain ").nth(1).unwrap();
        if contents == "no other bags." {
            continue;
        }

        let mut contained = Vec::new();

        for consist in contents.strip_suffix('.').unwrap().split(", ") {
            let consist = consist.rsplitn(2, ' ').nth(1).unwrap();
            let consist: Vec<&str> = consist.splitn(2, ' ').collect();
            let count = u32::from_str(consist[0]).unwrap();
            let containee = consist[1].to_string();

            if let Some(set) = contained_by.get_mut(&containee) {
                set.insert(subject.clone());
            } else {
                let mut set = HashSet::new();
                set.insert(subject.clone());
                contained_by.insert(containee.clone(), set);
            }

            contained.push((count, containee));
        }

        container_tree.insert(subject, contained);
    }

    let mut iter_stack = Vec::new();
    let mut container_set = HashSet::new();
    if let Some(l) = contained_by.get("shiny gold") {
        iter_stack.push(l.iter());
    }

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
    println!("{} bags", count_bags(&container_tree, "shiny gold"));

    Ok(())
}
