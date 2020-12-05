use crate::utilities::read_as_string;
use std::fs::File;

struct BoardingPass {
    column: u32,
    raw: String,
    row: u32,
    seat_id: u32,
}

fn parse_row(row: &char) -> u32 {
    if *row == 'B' {
        1
    } else {
        0
    }
}

fn parse_column(column: &char) -> u32 {
    if *column == 'R' {
        1
    } else {
        0
    }
}

fn to_boarding_pass(raw: &str) -> BoardingPass {
    let row = (&raw[0..7])
        .chars()
        .fold(0, |acc, val| acc * 2 + parse_row(&val));

    let column = (&raw[7..10])
        .chars()
        .fold(0, |acc, val| acc * 2 + parse_column(&val));

    let seat_id = row * 8 + column;

    BoardingPass {
        column: column,
        raw: raw.to_string(),
        row: row,
        seat_id: seat_id,
    }
}

pub fn run() {
    let input = File::open("./src/day5.txt")
        .and_then(|file| read_as_string(file))
        .expect("failed to read input file");

    let boarding_passes = input
        .iter()
        .map(|b| to_boarding_pass(&b))
        .collect::<Vec<BoardingPass>>();

    for pass in &boarding_passes {
        println!(
            "{0}: row {1:>4}, column {2:>4}, seat ID {3:>4}",
            pass.raw, pass.row, pass.column, pass.seat_id
        );
    }

    // part 1
    let max_seat_id = (&boarding_passes).iter().map(|b| b.seat_id).max().unwrap();
    println!("Maximum seat 💺 id: {0}", max_seat_id);

    // part 2
    let min_seat_id = (&boarding_passes).iter().map(|b| b.seat_id).min().unwrap();
    let missing_seat = (min_seat_id..(max_seat_id + 1))
        .filter(|&s| !boarding_passes.iter().any(|b| b.seat_id == s))
        .next();
    println!("Found my seat 💺 id: {0}", missing_seat.unwrap());
}
