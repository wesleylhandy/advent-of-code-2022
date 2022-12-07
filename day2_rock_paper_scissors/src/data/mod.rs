use std::fs::File;
use std::io::{self, BufRead};

pub fn read_data_from_file() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("./data/input.txt")?;
    Ok(io::BufReader::new(file).lines())
}