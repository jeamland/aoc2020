use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

use crate::{get_field, PassportError};

enum Height {
    Centimetres(u32),
    Inches(u32),
}

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

enum EyeColour {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

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

#[allow(dead_code)]
pub struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: Height,
    hcl: u32,
    ecl: EyeColour,
    pid: String,
    cid: Option<String>,
}

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
