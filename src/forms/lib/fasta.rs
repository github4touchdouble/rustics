use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use crate::utils::btree::integrate_splits;


pub struct Fasta {
    pub nucleotides: Vec<FastaRecord>,
}

impl Fasta {
    pub fn new() -> Fasta {
        Fasta {
            nucleotides: vec![],
        }
    }

    pub fn file_read_in(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut res = Fasta::new();


        let mut cur_header = String::new();
        let mut cur_sequence = String::new();
        for line in reader.lines() {
            let line = line?;

            //Check if current line is header or sequence
            if line.starts_with('>') {
                if !cur_header.is_empty() {
                    let mut record = FastaRecord::new();
                    record.header = cur_header.clone();
                    record.sequence = cur_sequence.clone();
                    res.nucleotides.push(record);
                    cur_sequence.clear();
                }
                //Update header
                cur_header = line[1..].to_string();
            } else {
                // new sequence
                cur_sequence.push_str(&line);
            }
        }
        //last line reached
        if !cur_header.is_empty() {
            let mut record = FastaRecord::new();
            record.header = cur_header;
            record.sequence = cur_sequence;
            res.nucleotides.push(record);
        }
        Ok(res)
    }

    pub fn search_header(&self, header: &str) -> Option<FastaRecord> {
        for record in &self.nucleotides{
            if record.header == header {
                let mut bin = FastaRecord::new();
                bin.header = record.header.clone();
                bin.sequence = record.sequence.clone();
                return Some(bin);
            }
        }
        None
    }

    pub fn search_sequence(&self, sequence: &str) -> Option<FastaRecord> {
        for record in &self.nucleotides{
            if record.sequence == sequence {
                let mut bin = FastaRecord::new();
                bin.header = record.header.clone();
                bin.sequence = record.sequence.clone();
                return Some(bin);
            }
        }
        None
    }

    pub fn count(&self) -> usize {
        *&self.nucleotides.len()
    }

    pub fn deep_count(&self) -> usize {
        let mut count:usize = 0;
        for rec in &self.nucleotides{
            count = count + rec.count().clone();
        }
        count
    }

    pub fn deep_split(&self) -> BTreeMap<char,u64> {
        let mut res:BTreeMap<char,u64> = BTreeMap::new();
        for rec in &self.nucleotides{
            let split = rec.split();
            res = integrate_splits(res,split);
        }
        res
    }
}


pub struct FastaRecord {
    pub header: String,
    pub sequence: String,
}

impl FastaRecord {
    pub fn new() -> FastaRecord {
        FastaRecord {
            header: String::new(),
            sequence: String::new(),
        }
    }

    pub fn set_header(&mut self, header: String) {
        self.header = header;
    }

    pub fn set_sequence(&mut self, sequence: String) {
        self.sequence = sequence;
    }

    pub fn header(&self) -> &str {
        &self.header
    }

    pub fn sequence(&self) -> &str {
        &self.header
    }

    pub fn count(&self) -> usize {
        *&self.sequence.len()
    }

    pub fn split(&self) -> BTreeMap<char,u64>{
        let mut res:BTreeMap<char,u64> = BTreeMap::new();
        for i in self.sequence.chars(){
            let count = res.entry(i).or_insert(0);
            *count += 1;
        }
        res
    }
}