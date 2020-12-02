use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

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
