use crate::utilities::read_as_string;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;

type Edge = (String, usize); // (target_node, number_of_bags)
type Graph = HashMap<String, HashSet<Edge>>; // (node, outgoing_edges)

fn parse_input(input: &Vec<String>) -> (Graph, Graph) {
    let mut parents = Graph::new();
    let mut children = Graph::new();

    // sample line: light violet bags contain 1 dotted red bag, 4 dim aqua bags.
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<count>\d) (?P<color>[a-z ]*) bag[s, ]*").expect("Invalid regexp");
    }

    for line in input {
        let split: Vec<&str> = line.split(" bags contain ").collect();
        let parent_color = split[0];

        RE.captures_iter(split[1]).for_each(|c| {
            let child_color: &str = &c["color"];

            children
                .entry(parent_color.to_string())
                .or_insert(HashSet::new())
                .insert((
                    child_color.to_string(),
                    c["count"].parse::<usize>().unwrap(),
                ));

            parents
                .entry(child_color.to_string())
                .or_insert(HashSet::new())
                .insert((parent_color.to_string(), 1));
        });
    }

    (parents, children)
}

pub fn run() {
    let input = File::open("./src/day7.txt")
        .and_then(|file| read_as_string(file))
        .expect("failed to read input file");

    let (parents, children) = parse_input(&input);

    // part 1

    fn traverse_parents(_color: &str, _graph: &Graph) -> usize {
        // TODO with this new approach
        0
    }

    println!(
        "Number of 🧳 my shiny gold bag could be in: {:}",
        traverse_parents("shiny gold", &parents)
    );

    // part 2

    fn traverse_children(color: &str, graph: &Graph) -> usize {
        match graph.get(color) {
            Some(edges) => edges
                .iter()
                .map(|(child_color, count)| count * (1 + traverse_children(child_color, graph)))
                .sum(),
            None => 0,
        }
    }

    println!(
        "Number of 🧳 my shiny gold bag must contain: {:}",
        traverse_children("shiny gold", &children)
    );
}
