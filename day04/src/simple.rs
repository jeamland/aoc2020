use std::collections::HashMap;
use std::convert::TryFrom;

use crate::{get_field, PassportError};

#[allow(dead_code)]
pub struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

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
