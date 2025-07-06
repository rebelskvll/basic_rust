use csv::{ReaderBuilder};
use std::{fs};

// Declare a string constant 
const FILENAME: &str = "src/data.csv";

fn main() {
    
    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = 
    ReaderBuilder::new().delimiter(b',').from_reader(content.as_bytes());

    for result in rdr.records() {
        println!("{}", result.unwrap().get(0).unwrap().trim());
    }
}
