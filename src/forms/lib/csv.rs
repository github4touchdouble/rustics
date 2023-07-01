use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::vec;
use csv::ReaderBuilder;

#[derive(Clone)]
pub struct Csv {
    pub name: String,
    pub lines: Vec<CsvRecord>,
}

impl Eq for Csv {}

impl PartialEq<Self> for Csv {
    fn eq(&self, other: &Self) -> bool {
        self.lines == other.lines
    }
}

impl PartialOrd<Self> for Csv {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.lines.partial_cmp(&other.lines)
    }
}

impl Ord for Csv {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.lines.cmp(&other.lines)
    }
}


impl Csv {
    pub fn new() -> Csv {
        Csv {
            name:String::new(),
            lines: vec![],
        }
    }

    pub fn file_read_in(name: &str, file_path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        let mut res = Csv::new();
        res.name = name.to_string();
        file.read_to_string(&mut contents)?;
        let mut reader = ReaderBuilder::new().has_headers(false).from_reader(contents.as_bytes());

        for result in reader.records() {
            let record = result?;
            let mut csv_record = CsvRecord::new();
            for field in record.iter() {
                let fields: Vec<String> = field.split(',').map(|s| s.to_owned()).collect();
                csv_record.rows.extend(fields);
            }
            res.lines.push(csv_record);
        }
        Ok(res)
    }

    pub fn without(&self, lines_to_remove: impl IntoIterator<Item = std::ops::Range<usize>>) -> Csv {
        let mut bin = self.lines.clone();

        for range in lines_to_remove {
            let start = range.start;
            let end = range.end.min(bin.len());

            bin.drain(start..end);
        }

        Csv {
            name: self.name.clone(),
            lines: bin,
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct CsvRecord {
    pub rows: Vec<String>,
}

impl CsvRecord {
    pub fn new() -> CsvRecord{
        CsvRecord{
            rows: vec![],
        }
    }



}