use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use serde_json::Value::String;

pub struct Fasta {
    nucleotides: Vec<Record>,
}

impl Fasta {
    pub fn new() -> Fasta{
        Fasta{
            nucleotides: vec![],
        }
    }

    pub fn file_read_in(file_path: &str) -> Result<Self, std::io::Error>{
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut res = Fasta::new();


        let mut cur_header = String::new();
        let mut cur_sequence = String::new;
        for line in reader.lines(){
            let line = line?;

            //Check if current line is header or sequence
            if line.starts_with('>') {
                if !cur_header.is_empty() || !cur_sequence.is_empty(){
                    panic!("Error 1: Fasta::file_read_in()")
                }
                //new header found
                cur_header = line[1..].to_string();
                continue;
            }
            else {
                if cur_header.is_empty() || !cur_sequence.is_empty(){
                    panic!("Error 2: Fasta::file_read_in()")
                }
                //sequence found
                cur_sequence = line.to_string();
                bin = Record::new();
                bin.header = cur_header;
                bin.sequence = cur_sequence;
                res.nucleotides.push(bin);
                cur_sequence = String::new();
                cur_header = String::new();
            }

        }
        Ok(res)
    }

}


pub struct Record {
    header: String,
    sequence: String,
}

impl Record {
    pub fn new() -> Record{
        Record{
            header: String::new(),
            sequence: String::new(),
        }
    }

    pub fn set_header(&mut self, header: String){
        self.header = header;
    }

    pub fn set_sequence(&mut self, sequence: String){
        self.sequence = sequence;
    }

    pub fn header(&self) -> &str{
        &self.header
    }

    pub fn sequence(&self) -> &str{
        &self.header
    }



}