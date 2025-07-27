use core::fmt;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
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

impl<T: Serialize + Clone> Record<T> {
    pub fn new(record_type: RecordType, data: T) -> Self {
        Self { record_type, data }
    }
    pub fn from_vec(objects: &Vec<T>) -> Vec<Self> {
        objects
            .iter()
            .map(|obj| Self::new(RecordType::Table, obj.clone()))
            .collect()
    }
}

pub struct File {
    file: fs::File,
}

impl File {
    pub fn open(path: &str) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Failed to open database file");

        Self { file }
    }

    pub fn load_records<T: fmt::Debug + Serialize + for<'de> Deserialize<'de>>(
        &mut self,
    ) -> Result<Vec<Record<T>>, Box<dyn std::error::Error>> {
        let mut buffer = Vec::new();
        self.file.read_to_end(&mut buffer)?;

        if buffer.is_empty() {
            return Ok(Vec::new());
        }

        let config = bincode::config::standard();
        let (records, _) = bincode::serde::decode_from_slice(&buffer, config)?;
        Ok(records)
    }

    pub fn write_records<T: Serialize>(
        &mut self,
        records: Vec<Record<T>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config = bincode::config::standard();
        let encoded = bincode::serde::encode_to_vec(&records, config)?;

        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(&encoded)?;
        self.file.set_len(encoded.len() as u64)?;

        Ok(())
    }
}
