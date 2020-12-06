use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

/// Joins multiple lines of strings into one line (space-separated) where a
/// a group is considered to be separated by an empty line.
pub fn join_lines(input: &Vec<String>) -> Vec<String> {
    let mut output = Vec::new();

    let mut grouped_line = String::new();
    for val in input.iter() {
        if val.len() == 0 {
            output.push(grouped_line.clone());
            grouped_line.clear();
        } else if grouped_line.len() == 0 {
            grouped_line.push_str(val);
        } else {
            grouped_line.push_str(&format!(" {}", val));
        }
    }
    output.push(grouped_line.clone());

    output
}

pub fn read_as_int64<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn read_as_string<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}
