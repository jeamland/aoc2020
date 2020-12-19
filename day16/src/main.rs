use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Range;
use std::str::FromStr;

use clap::{App, Arg};

#[derive(Debug, PartialEq)]
enum ParserState {
    Fields,
    Label,
    YourTicket,
    NearbyTickets,
}

fn valid_fields_for_position(
    fields: &HashMap<String, Vec<Range<usize>>>,
    values: Vec<usize>,
) -> HashSet<String> {
    let mut valid_fields = HashSet::new();

    for (field_name, ranges) in fields {
        let mut valid = true;
        for value in &values {
            let mut value_valid = false;
            for range in ranges {
                if range.contains(&value) {
                    value_valid = true;
                    break;
                }
            }

            if !value_valid {
                valid = false;
                break;
            }
        }

        if valid {
            valid_fields.insert(field_name.clone());
        }
    }

    valid_fields
}

fn main() -> std::io::Result<()> {
    let matches = App::new("AOC2020 Day 16")
        .arg(
            Arg::with_name("INPUT")
                .help("Input file name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = File::open(matches.value_of("INPUT").unwrap())?;
    let reader = BufReader::new(file);

    let lines = reader.lines();
    let mut state = ParserState::Fields;
    let mut fields = HashMap::new();
    let mut ticket: Vec<usize> = Vec::new();
    let mut nearby_tickets: Vec<Vec<usize>> = Vec::new();

    for line in lines {
        let line = line.unwrap();

        match line.as_str() {
            "" => {
                state = ParserState::Label;
                continue;
            }
            "your ticket:" => {
                state = ParserState::YourTicket;
                continue;
            }
            "nearby tickets:" => {
                state = ParserState::NearbyTickets;
                continue;
            }
            _ => (),
        };

        if state == ParserState::Fields {
            let parts: Vec<&str> = line.split(": ").collect();
            let field_name = parts[0].to_string();
            let mut ranges = Vec::new();
            for range in parts[1].split(" or ") {
                let ends: Vec<usize> = range
                    .split('-')
                    .map(|v| usize::from_str(v).unwrap())
                    .collect();
                ranges.push(ends[0]..ends[1] + 1);
            }
            fields.insert(field_name, ranges);
        } else if state == ParserState::YourTicket {
            ticket = line
                .split(',')
                .map(|v| usize::from_str(v).unwrap())
                .collect();
        } else if state == ParserState::NearbyTickets {
            let nt = line
                .split(',')
                .map(|v| usize::from_str(v).unwrap())
                .collect();
            nearby_tickets.push(nt);
        }
    }

    let mut error_rate = 0;
    let mut valid_tickets = Vec::new();

    for nt in nearby_tickets {
        let mut valid_ticket = true;

        for value in nt.iter() {
            let mut valid = false;

            for ranges in fields.values() {
                for range in ranges {
                    if range.contains(&value) {
                        valid = true;
                        break;
                    }
                }

                if valid {
                    break;
                }
            }

            if !valid {
                error_rate += value;
                valid_ticket = false;
            }
        }

        if valid_ticket {
            valid_tickets.push(nt);
        }
    }

    println!("error_rate: {}", error_rate);
    println!();

    let mut valid_fields = Vec::new();

    for position in 0..fields.len() {
        let values: Vec<usize> = valid_tickets.iter().map(|t| t[position]).collect();
        let valid = valid_fields_for_position(&fields, values);
        valid_fields.push(valid);
    }

    let mut fixed_fields = HashSet::new();

    loop {
        let singleton = valid_fields
            .iter()
            .find(|v| v.len() == 1 && v.intersection(&fixed_fields).count() == 0)
            .unwrap()
            .clone();
        valid_fields = valid_fields
            .iter()
            .map(|v| {
                if v.len() == 1 {
                    v.clone()
                } else {
                    let new_v: HashSet<String> = v.difference(&singleton).cloned().collect();
                    new_v
                }
            })
            .collect();
        fixed_fields.insert(singleton.iter().next().unwrap().clone());

        let mut all_fixed = true;
        for field in valid_fields.iter() {
            if field.len() > 1 {
                all_fixed = false;
                break;
            }
        }
        if all_fixed {
            break;
        }
    }

    let field_order: Vec<String> = valid_fields
        .iter()
        .map(|v| v.iter().next().unwrap().clone())
        .collect();
    println!("{:?}", field_order);

    let ticket: HashMap<String, usize> = field_order
        .iter()
        .cloned()
        .zip(ticket.iter().copied())
        .collect();

    println!("{:?}", ticket);

    let departure_product: usize = ticket
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| *v)
        .product();

    println!("departure sum: {}", departure_product);

    Ok(())
}
