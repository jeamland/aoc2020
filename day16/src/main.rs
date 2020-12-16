use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

use clap::{App, Arg};

#[derive(Debug, PartialEq)]
enum ParserState {
    Fields,
    Label,
    YourTicket,
    NearbyTickets,
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
        } else if state == ParserState::NearbyTickets {
            let nt = line
                .split(',')
                .map(|v| usize::from_str(v).unwrap())
                .collect();
            nearby_tickets.push(nt);
        }
    }

    let mut error_rate = 0;

    for nt in nearby_tickets {
        for value in nt {
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
            }
        }
    }

    println!("{}", error_rate);

    Ok(())
}
