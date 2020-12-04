use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[cfg(feature = "strict")]
use std::str::FromStr;

use clap::{App, Arg};

#[cfg(feature = "simple")]
#[allow(dead_code)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

#[cfg(feature = "strict")]
enum Height {
    Centimetres(u32),
    Inches(u32),
}

#[cfg(feature = "strict")]
impl TryFrom<&str> for Height {
    type Error = PassportError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(h) = value.strip_suffix("cm") {
            let h = u32::from_str(h)?;
            if h < 150 || h > 193 {
                return Err(PassportError::BadValue);
            }
            Ok(Self::Centimetres(h))
        } else if let Some(h) = value.strip_suffix("in") {
            let h = u32::from_str(h)?;
            if h < 59 || h > 76 {
                return Err(PassportError::BadValue);
            }
            Ok(Self::Inches(h))
        } else {
            Err(PassportError::BadValue)
        }
    }
}

#[cfg(feature = "strict")]
enum EyeColour {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

#[cfg(feature = "strict")]
impl TryFrom<&str> for EyeColour {
    type Error = PassportError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "amb" => Ok(Self::Amber),
            "blu" => Ok(Self::Blue),
            "brn" => Ok(Self::Brown),
            "gry" => Ok(Self::Grey),
            "grn" => Ok(Self::Green),
            "hzl" => Ok(Self::Hazel),
            "oth" => Ok(Self::Other),
            _ => Err(PassportError::BadValue),
        }
    }
}

#[cfg(feature = "strict")]
#[allow(dead_code)]
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: Height,
    hcl: u32,
    ecl: EyeColour,
    pid: String,
    cid: Option<String>,
}

#[derive(Debug)]
enum PassportError {
    MissingField(String),
    BadValue,
}

impl From<std::num::ParseIntError> for PassportError {
    fn from(_error: std::num::ParseIntError) -> Self {
        Self::BadValue
    }
}

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

#[cfg(feature = "simple")]
impl TryFrom<HashMap<String, String>> for Passport {
    type Error = PassportError;

    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        let byr = get_field!(map, "byr");
        let iyr = get_field!(map, "iyr");
        let eyr = get_field!(map, "eyr");
        let hgt = get_field!(map, "hgt");
        let hcl = get_field!(map, "hcl");
        let ecl = get_field!(map, "ecl");
        let pid = get_field!(map, "pid");
        let cid = map.get("cid").map(|v| v.to_owned());

        Ok(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        })
    }
}

#[cfg(feature = "strict")]
impl TryFrom<HashMap<String, String>> for Passport {
    type Error = PassportError;

    fn try_from(map: HashMap<String, String>) -> Result<Self, Self::Error> {
        let byr = u32::from_str(get_field!(map, "byr", 4).as_str())?;
        let iyr = u32::from_str(get_field!(map, "iyr", 4).as_str())?;
        let eyr = u32::from_str(get_field!(map, "eyr", 4).as_str())?;
        let hgt = Height::try_from(get_field!(map, "hgt").as_str())?;
        let hcl = get_field!(map, "hcl", 7);
        let ecl = EyeColour::try_from(get_field!(map, "ecl", 3).as_str())?;
        let pid = get_field!(map, "pid", 9);
        let cid = map.get("cid").map(|v| v.to_owned());

        if byr < 1920 || byr > 2002 {
            return Err(PassportError::BadValue);
        }

        if iyr < 2010 || iyr > 2020 {
            return Err(PassportError::BadValue);
        }

        if eyr < 2020 || eyr > 2030 {
            return Err(PassportError::BadValue);
        }

        let hcl = if let Some(c) = hcl.strip_prefix('#') {
            u32::from_str_radix(c, 16)?
        } else {
            return Err(PassportError::BadValue);
        };

        u32::from_str(&pid)?;

        Ok(Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        })
    }
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
