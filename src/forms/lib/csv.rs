use std::fs::File;
use std::io::Read;
use std::vec;
use csv::ReaderBuilder;

#[derive(Clone)]
pub struct Csv {
    pub lines: Vec<CsvRecord>,
}

impl Csv {
    pub fn new() -> Csv {
        Csv {
            lines: vec![],
        }
    }

    pub fn file_read_in(file_path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        let mut res = Csv::new();
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
            lines: bin,
        }
    }
}

#[derive(Clone)]
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