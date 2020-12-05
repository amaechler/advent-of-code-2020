use crate::utilities::read_as_string;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;

#[derive(Debug)]
struct Height {
    height: u32,
    unit: Unit,
}

#[derive(Debug)]
enum Unit {
    Centimeters,
    Inches,
}

#[derive(Debug)]
struct Passport {
    birth_year: Result<u32, String>,
    expiration_year: Result<u32, String>,
    eye_color: Result<String, String>,
    hair_color: Result<String, String>,
    height: Result<Height, String>,
    issue_year: Result<u32, String>,
    passport_id: Result<u32, String>,
}

fn is_passport_valid_lazy(passport: &str) -> bool {
    lazy_static! {
        static ref RE_BYR: Regex = Regex::new(r"byr:([#0-9a-z]+)").unwrap();
        static ref RE_ECL: Regex = Regex::new(r"ecl:([#0-9a-z]+)").unwrap();
        static ref RE_EYR: Regex = Regex::new(r"eyr:([#0-9a-z]+)").unwrap();
        static ref RE_HCL: Regex = Regex::new(r"hcl:([#0-9a-z]+)").unwrap();
        static ref RE_HGT: Regex = Regex::new(r"hgt:([#0-9a-z]+)").unwrap();
        static ref RE_IYR: Regex = Regex::new(r"iyr:([#0-9a-z]+)").unwrap();
        static ref RE_PID: Regex = Regex::new(r"pid:([#0-9a-z]+)").unwrap();
    }

    RE_BYR.is_match(passport)
        && RE_ECL.is_match(passport)
        && RE_EYR.is_match(passport)
        && RE_HCL.is_match(passport)
        && RE_HGT.is_match(passport)
        && RE_IYR.is_match(passport)
        && RE_PID.is_match(passport)
}

fn parse_passport(passport: &str) -> Passport {
    lazy_static! {
        static ref RE_BYR: Regex = Regex::new(r"byr:(\d{4})").unwrap();
        static ref RE_ECL: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
        static ref RE_EYR: Regex = Regex::new(r"eyr:(\d{4})").unwrap();
        static ref RE_HCL: Regex = Regex::new(r"hcl:(#[0-9a-f]{6})").unwrap();
        static ref RE_HGT_NUM: Regex = Regex::new(r"(\d+)").unwrap();
        static ref RE_HGT: Regex = Regex::new(r"hgt:(\d+(in|cm))").unwrap();
        static ref RE_IYR: Regex = Regex::new(r"iyr:(\d{4})").unwrap();
        static ref RE_PID: Regex = Regex::new(r"pid:(\d+)").unwrap();
    }

    Passport {
        birth_year: RE_BYR
            .captures(passport)
            .ok_or("Required value".to_string())
            .map(|c| c.get(1).ok_or("Required value".to_string()))
            .and_then(|m| {
                m.unwrap()
                    .as_str()
                    .parse::<u32>()
                    .map_err(|e| format!("Invalid value {}", e))
            })
            .and_then(|x| {
                if 1920 <= x && x <= 2002 {
                    Ok(x)
                } else {
                    Err(format!("Invalid value {}", x))
                }
            }),

        expiration_year: RE_EYR
            .captures(passport)
            .ok_or("Required value".to_string())
            .map(|c| c.get(1).ok_or("Required value".to_string()))
            .and_then(|m| {
                m.unwrap()
                    .as_str()
                    .parse::<u32>()
                    .map_err(|e| format!("Invalid value {}", e))
            })
            .and_then(|e| {
                if 2020 <= e && e <= 2030 {
                    Ok(e)
                } else {
                    Err(format!("Invalid value {}", e))
                }
            }),

        eye_color: RE_ECL
            .captures(passport)
            .ok_or("Required value".to_string())
            .map(|c| c.get(1).ok_or("Required value".to_string()))
            .map(|m| m.unwrap().as_str().to_string()),

        hair_color: RE_HCL
            .captures(passport)
            .ok_or("Required value".to_string())
            .map(|c| c.get(1).ok_or("Required value".to_string()))
            .map(|m| m.unwrap().as_str().to_string()),

        height: RE_HGT
            .captures(passport)
            .ok_or("Required value".to_string())
            .map(|c| c.get(1).ok_or("Required value".to_string()))
            .and_then(|m| Ok(m.unwrap().as_str().to_string()))
            .and_then(|h| {
                let height = (&h[0..h.len() - 2]).parse::<u32>().unwrap();
                if h.ends_with("cm") {
                    Ok(Height {
                        height: height,
                        unit: Unit::Centimeters,
                    })
                } else {
                    Ok(Height {
                        height: height,
                        unit: Unit::Inches,
                    })
                }
            })
            .and_then(|h| match h.unit {
                Unit::Centimeters => {
                    if 150 <= h.height && h.height <= 193 {
                        Ok(h)
                    } else {
                        Err(format!("Invalid value {}", h.height))
                    }
                }
                Unit::Inches => {
                    if 59 <= h.height && h.height <= 76 {
                        Ok(h)
                    } else {
                        Err(format!("Invalid value {}", h.height))
                    }
                }
            }),

        issue_year: RE_IYR
            .captures(passport)
            .ok_or("Required value".to_string())
            .map(|c| c.get(1).ok_or("Required value".to_string()))
            .and_then(|m| {
                m.unwrap()
                    .as_str()
                    .parse::<u32>()
                    .map_err(|e| format!("Invalid value {}", e))
            })
            .and_then(|i| {
                if 2010 <= i && i <= 2020 {
                    Ok(i)
                } else {
                    Err(format!("Invalid value {}", i))
                }
            }),

        passport_id: RE_PID
            .captures(passport)
            .ok_or("Required value".to_string())
            .map(|c| c.get(1).ok_or("Required value".to_string()))
            .and_then(|m| {
                let s = m.unwrap().as_str();
                match s.len() {
                    9 => s.parse::<u32>().map_err(|e| format!("Invalid value {}", e)),
                    _ => Err(format!("Invalid value {}", s)),
                }
            }),
    }
}

pub fn run() {
    let input = File::open("./src/day4.txt")
        .and_then(|file| read_as_string(file))
        .expect("failed to read input file");

    // TODO: think about how to make this normalization more elegant
    let mut normalized_passports = Vec::new();
    let mut single_passport = String::new();
    for val in input.iter() {
        if val == "" {
            normalized_passports.push(single_passport.clone());
            single_passport.clear();
        } else {
            single_passport.push_str(&format!(" {}", val));
        }
    }
    normalized_passports.push(single_passport.clone());

    // part 1

    let number_of_valid_passports = normalized_passports
        .iter()
        .filter(|p| is_passport_valid_lazy(p))
        .count();

    println!(
        "Number of valid 🛂 passports: {0} (out of {1})",
        number_of_valid_passports,
        normalized_passports.len()
    );

    // part 2

    let parsed_passports = normalized_passports
        .iter()
        .map(|p| parse_passport(p))
        .collect::<Vec<Passport>>();

    let number_of_valid_passports = parsed_passports
        .iter()
        .filter(|p| {
            p.birth_year.is_ok()
                && p.expiration_year.is_ok()
                && p.eye_color.is_ok()
                && p.hair_color.is_ok()
                && p.height.is_ok()
                && p.issue_year.is_ok()
                && p.passport_id.is_ok()
        })
        .count();

    println!(
        "Number of valid 🛂 passports: {0} (out of {1})",
        number_of_valid_passports,
        normalized_passports.len()
    );
}
