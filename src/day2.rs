use crate::utilities::read_as_string;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;

struct PasswordPolicy {
    lower_limit: usize,
    upper_limit: usize,
    character: char,
    password: String,
}

fn parse_password_definition(definition: &str) -> PasswordPolicy {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): (.*)$").unwrap();
    }

    RE.captures(definition)
        .map(|caps| {
            let lower_limit = caps
                .get(1)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .expect("Failed to parse lower limit");

            let upper_limit = caps
                .get(2)
                .map(|m| m.as_str().parse::<usize>().unwrap())
                .expect("Failed to parse upper limit");

            let character = caps
                .get(3)
                .map(|m| m.as_str().chars().nth(0).unwrap())
                .expect("Failed to parse rule character");

            let password = caps
                .get(4)
                .map(|m| m.as_str())
                .expect("Failed to parse password");

            PasswordPolicy {
                lower_limit: lower_limit,
                upper_limit: upper_limit,
                character: character,
                password: String::from(password),
            }
        })
        .unwrap()
}

pub fn run() {
    let input = File::open("./src/day2.txt")
        .and_then(|file| read_as_string(file))
        .expect("failed to read input file");

    let password_definitions = input
        .iter()
        .map(|s| parse_password_definition(s))
        .collect::<Vec<PasswordPolicy>>();

    // part 1

    let number_of_passwords_valid = password_definitions
        .iter()
        .filter(|p| {
            let number_of_times_character_is_in_password =
                p.password.chars().filter(|c| *c == p.character).count();

            number_of_times_character_is_in_password >= p.lower_limit
                && number_of_times_character_is_in_password <= p.upper_limit
        })
        .count();

    println!("Number of passwords total: {0}", input.len());
    println!(
        "Number of passwords valid (1): {0}",
        number_of_passwords_valid
    );

    // part 2

    let number_of_passwords_valid = password_definitions
        .iter()
        .filter(|p| {
            let a = p.password.chars().nth(p.lower_limit - 1).unwrap();
            let b = p.password.chars().nth(p.upper_limit - 1).unwrap();

            (a == p.character) ^ (b == p.character)
        })
        .count();

    println!(
        "Number of passwords valid (2): {0}",
        number_of_passwords_valid
    );
}
