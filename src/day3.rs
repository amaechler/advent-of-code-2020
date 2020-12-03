use crate::utilities::read_as_string;
use std::fs::File;

fn is_tree(c: char) -> i32 {
    if c == '#' {
        1
    } else {
        0
    }
}

fn ride_the_slopes(input: &Vec<String>, right: usize, down: usize) -> i32 {
    input
        .iter()
        .enumerate()
        .filter_map(
            |(index, value)| {
                if index % down == 0 {
                    Some(value)
                } else {
                    None
                }
            },
        )
        .fold((0, 0usize), |acc, val| {
            (
                acc.0 + is_tree(val.chars().nth(acc.1).unwrap()), // number of trees
                (acc.1 + right) % 31,                             // next position
            )
        })
        .0
}

pub fn run() {
    let input = File::open("./src/day3.txt")
        .and_then(|file| read_as_string(file))
        .expect("failed to read input file");

    // part 1

    let number_of_trees = ride_the_slopes(&input, 3, 1);
    println!("Number of trees encountered: {0}", number_of_trees);

    // part 2

    let number_of_trees = vec![
        ride_the_slopes(&input, 1, 1),
        ride_the_slopes(&input, 3, 1),
        ride_the_slopes(&input, 5, 1),
        ride_the_slopes(&input, 7, 1),
        ride_the_slopes(&input, 1, 2),
    ];

    println!("Number of trees encountered: {:?}", number_of_trees);
    println!(
        "Number of trees multiplied: {:?}",
        number_of_trees
            .iter()
            .fold(1u64, |acc, val| acc * *val as u64)
    );
}
