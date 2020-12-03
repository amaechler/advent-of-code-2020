use std::env;

mod day1;
mod day2;
mod day3;
mod utilities;

fn help() {
    println!(
        "Advent of Code 2020

USAGE:
    advent-of-code-2020 [DAY]

EXAMPLES:
    $ ./advent-of-code-2020 day1
    Runs the first day of advent of code.
"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        2 => {
            let day_to_run = &args[1];
            match day_to_run.as_str() {
                "day1" => day1::run(),
                "day2" => day2::run(),
                "day3" => day3::run(),
                x => println!("Unknown day '{0}'", x),
            }
        }
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
