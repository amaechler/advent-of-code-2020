use crate::utilities::read;
use std::fs::File;

pub fn run() {
    let input = File::open("./src/day1.txt")
        .and_then(|file| read(file))
        .expect("failed to read input file");

    // part 1

    let mut tuple = (0, 0);
    for i in 0..input.len() {
        for j in i..input.len() {
            if input[i] + input[j] == 2020 {
                tuple = (input[i], input[j]);
                break;
            }
        }
    }

    println!("{} + {} = 2020", tuple.0, tuple.1);
    println!("{} * {} = {}", tuple.0, tuple.1, tuple.0 * tuple.1);

    // part 2

    let mut triple = (0, 0, 0);
    for i in 0..input.len() {
        for j in i..input.len() {
            for k in j..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    triple = (input[i], input[j], input[k]);
                    break;
                }
            }
        }
    }

    println!("{} + {} + {} = 2020", triple.0, triple.1, triple.2);
    println!(
        "{} * {} * {} = {}",
        triple.0,
        triple.1,
        triple.2,
        triple.0 * triple.1 * triple.2
    );
}
