use std::fs;
use std::io::{BufRead, BufReader, Error};

pub fn read_groups(filename: &str) -> Result<Vec<Vec<String>>, Error> {
    let buff_reader = BufReader::new(fs::File::open(filename)?);
    let mut read_groups: Vec<Vec<String>> = vec![];
    for line in buff_reader.lines() {
        let line = line.unwrap();
        let parsed: Vec<_> = line.split(":").into_iter().map(|s| s.to_string()).collect();
        read_groups.push(parsed);
    }
    Ok(read_groups)
}


