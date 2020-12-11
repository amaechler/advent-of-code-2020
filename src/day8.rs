use crate::utilities::read_as_string;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^(?P<operation>\w{3}) (?P<argument>[+-]\d+)$").expect("Invalid regexp");
    static ref RE_ACC: Regex = Regex::new(r"^(?P<operation>\w{3})").expect("Invalid regexp");
}

fn run_assembly_code(input: &Vec<String>) -> (i32, usize) {
    let mut visited_positions = HashSet::new();

    let mut accumulator = 0;
    let mut pos: usize = 0;
    loop {
        let caps = RE.captures(&input[pos]).unwrap();
        let argument = &caps["argument"].parse::<i32>().unwrap();

        match &caps["operation"] {
            "acc" => {
                accumulator += argument;
                pos += 1;
            }
            "nop" => {
                pos += 1;
            }
            "jmp" => {
                // check if illegal position
                if (pos as i32 + argument) < 0 {
                    break;
                }

                pos = (pos as i32 + argument) as usize;
            }
            o => println!("Unknown operator '{}'", o),
        }

        // check if program completed
        if pos >= input.len() {
            break;
        }

        // check if infinite loop
        if visited_positions.contains(&pos) {
            break;
        } else {
            visited_positions.insert(pos);
        }
    }

    (accumulator, pos)
}

pub fn run() {
    let input = File::open("./src/day8.txt")
        .and_then(|file| read_as_string(file))
        .expect("failed to read input file");

    // part 1

    let (accumulator, _) = run_assembly_code(&input);
    println!("Last value of accumulator before ♾️ loop: {:}", accumulator);

    // part 2

    for i in 0..input.len() {
        let mut clone = input.clone();
        if clone[i].contains("nop") {
            clone[i] = clone[i].replace("nop", "jmp");
        } else if clone[i].contains("jmp") {
            clone[i] = clone[i].replace("jmp", "nop");
        }

        let (accumulator, pos) = run_assembly_code(&clone);
        if pos >= clone.len() {
            println!("Replaced pos {} to break ♾️ loop", i);
            println!("Last value of accumulator before ♾️ loop: {:}", accumulator);
            break;
        }
    }
}
