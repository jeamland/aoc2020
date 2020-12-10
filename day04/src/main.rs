use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};

mod simple;
mod strict;

#[cfg(feature = "simple")]
use crate::simple::Passport;

#[cfg(feature = "strict")]
use crate::strict::Passport;

#[derive(Debug)]
pub enum PassportError {
    MissingField(String),
    BadValue,
}

impl From<std::num::ParseIntError> for PassportError {
    fn from(_error: std::num::ParseIntError) -> Self {
        Self::BadValue
    }
}

#[macro_export]
macro_rules! get_field {
    ($map:ident, $name:expr) => {
        match $map.get($name) {
            Some(v) => v.to_owned(),
            None => return Err(PassportError::MissingField($name.to_string())),
        }
    };
    ($map:ident, $name:expr, $len:expr) => {
        match $map.get($name) {
            Some(v) => {
                if v.len() != $len {
                    return Err(PassportError::BadValue);
                } else {
                    v.to_owned()
                }
            }
            None => return Err(PassportError::MissingField($name.to_string())),
        }
    };
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

    let mut data: HashMap<String, String> = HashMap::new();
    let mut valid = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            if !data.is_empty() {
                if Passport::try_from(data).is_ok() {
                    valid += 1;
                };

                data = HashMap::new();
            }

            continue;
        }

        for entry in line.split_whitespace() {
            let parts: Vec<&str> = entry.split(':').collect();
            let key = parts[0].to_string();
            let value = parts[1].to_string();

            data.insert(key, value);
        }
    }

    if !data.is_empty() && Passport::try_from(data).is_ok() {
        valid += 1;
    };

    println!("{} valid", valid);

    Ok(())
}
