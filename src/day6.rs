use crate::utilities::join_lines;
use crate::utilities::read_as_string;
use std::collections::HashSet;
use std::fs::File;

pub fn run() {
    let input = File::open("./src/day6.txt")
        .and_then(|file| read_as_string(file))
        .expect("failed to read input file");

    let grouped_custom_forms = join_lines(&input);

    // part 1

    let number_of_yes_answers: usize = grouped_custom_forms
        .iter()
        .map(|g| {
            g.chars()
                .filter(|c| *c != ' ')
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();

    println!(
        "Total number of YES 📝 answers (ANY): {:?}",
        number_of_yes_answers
    );

    // part 2

    let number_of_yes_answers: usize = grouped_custom_forms
        .iter()
        .map(|x| {
            let group_sets: Vec<HashSet<_>> = x
                .chars()
                .collect::<String>()
                .split_whitespace()
                .map(|x| x.chars().collect())
                .collect();

            group_sets
                .iter()
                .skip(1)
                .fold(group_sets[0].clone(), |acc, val| {
                    acc.intersection(val).cloned().collect()
                })
                .len()
        })
        .sum();

    println!(
        "Total number of YES 📝 answers (ALL): {0}",
        number_of_yes_answers
    );
}
