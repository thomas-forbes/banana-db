use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum RecordType {
    Table,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Record<T: Serialize> {
    pub record_type: RecordType,
    pub data: T,
}

impl<T: Serialize> Record<T> {
    pub fn new(record_type: RecordType, data: T) -> Self {
        Self { record_type, data }
    }
}

pub struct File<T: Serialize> {
    file: fs::File,
    records: Vec<Record<T>>,
}

impl<T: Serialize + for<'de> Deserialize<'de>> File<T> {
    pub fn open(path: String) -> Self {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Failed to open database file");

        let records = Self::load_records(&mut file).expect("Failed to read records");

        Self { file, records }
    }

    fn load_records(file: &mut fs::File) -> Result<Vec<Record<T>>, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        if buffer.is_empty() {
            return Ok(Vec::new());
        }

        let config = bincode::config::standard();
        let (records, _) = bincode::serde::decode_from_slice(&buffer, config)?;
        Ok(records)
    }

    fn flush_records(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let config = bincode::config::standard();
        let encoded = bincode::serde::encode_to_vec(&self.records, config)?;
        self.file.write_all(&encoded)?;
        Ok(())
    }

    pub fn get_records(self) -> Vec<Record<T>> {
        self.records
    }

    pub fn add_record(&mut self, record: Record<T>) {
        self.records.push(record);
        self.flush_records().unwrap();
    }
}
