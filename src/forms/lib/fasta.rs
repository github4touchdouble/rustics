use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub struct Record {
    header: String,
    sequence: String,
}

impl Record {
    fn new() -> Record{
        Record {
            header: String::new(),
            sequence: String::new(),
        }
    }

    fn read_in(file_path: &str) -> io::Result<Vec<Record>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut records = vec![];
        let mut current = Record::new();

        for line in reader.lines(){
            let line = line?;
            if line.starts_with('>') {
                if !current.header.is_empty(){
                    records.push(current);
                    current = Record::new();
                }
                current.header = line[1..].trim().to_string();
            } else {
                current.sequence.push_str(&line);
            }
        }
        if !current.header.is_empty(){
            records.push(current);
        }
        Ok(records)
    }

    fn print(&self) {
        println!("Header: {}", self.header);
        println!("Sequence: {}", self.sequence);
        println!();
    }
}